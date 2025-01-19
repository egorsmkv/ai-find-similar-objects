use std::error::Error;
use std::path::{Path, PathBuf};
use std::time::Instant;

use anyhow::{Context, Result};
use clap::Parser;
use image::GenericImageView;
use linemux::MuxedLines;
use ort::execution_providers::{CUDAExecutionProvider, CoreMLExecutionProvider};
use tracing::{debug, error, info, warn};
use yolo_rs::{YoloEntityOutput, image_to_yolo_input_tensor, inference, model};

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    /// A path to the files we `tail -f`
    #[arg(long)]
    images_file: PathBuf,

    /// A path to our ONNX YOLO model
    #[arg(long)]
    yolo_model: PathBuf,

    /// probability threshold
    #[arg(long)]
    probability_threshold: Option<f32>,

    /// iou threshold
    #[arg(long)]
    iou_threshold: Option<f32>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    debug!("Initializing ONNX runtime...");

    ort::init()
        .with_execution_providers([
            CUDAExecutionProvider::default().build(),
            CoreMLExecutionProvider::default().build(),
        ])
        .commit()?;

    info!("Load images path: {:?}", args.images_file.display());

    let mut lines = MuxedLines::new()?;

    lines.add_file(args.images_file).await?;

    info!("Loading model: {:?}", args.yolo_model.display());
    let model = {
        let mut model = model::YoloModelSession::from_filename_v8(&args.yolo_model)
            .with_context(|| format!("failed to load model {:?}", args.yolo_model.display()))?;

        model.iou_threshold = args.iou_threshold;
        model.probability_threshold = args.probability_threshold;

        model
    };

    while let Ok(Some(line)) = lines.next_line().await {
        let row = line.line().to_string();

        debug!("A new line: {}", row);

        if row.is_empty() {
            warn!("A line is empty");
            continue;
        }

        let path = Path::new(row.as_str());

        if !path.exists() {
            warn!("File does not exist: {:?}", path.display());

            continue;
        }

        info!("Loading image: {:?}", path.display());

        match image::open(path) {
            Ok(img) => {
                debug!("Converting image to tensor");
                let input = image_to_yolo_input_tensor(&img);

                info!("Running inference");

                let now = Instant::now();
                let result = inference(&model, input.view())?;

                info!("Inference took {:?}", now.elapsed());

                let (img_width, img_height) = (img.width(), img.height());

                debug!("Image resolution: {:?}x{:?}", img_width, img_height);

                for YoloEntityOutput {
                    bounding_box: bbox,
                    label,
                    confidence,
                } in result
                {
                    info!(
                        "Found entity {:?} with confidence {:.2} at ({:.2}, {:.2}) - ({:.2}, {:.2})",
                        label, confidence, bbox.x1, bbox.y1, bbox.x2, bbox.y2
                    );

                    let sub_image = img.view(
                        bbox.x1 as u32,
                        bbox.y1 as u32,
                        (bbox.x2 - bbox.x1) as u32,
                        (bbox.y2 - bbox.y1) as u32,
                    );
                    let extracted_img = sub_image.to_image();

                    let filename = format!(
                        "detections/entity_{}_{:.2}_{:.2}x{:.2}__{:.2}x{:.2}.png",
                        label, confidence, bbox.x1, bbox.y1, bbox.x2, bbox.y2
                    );

                    extracted_img.save(filename).unwrap();
                }
            }
            Err(e) => {
                error!("Error opening image: {:?}", e);
            }
        }
    }

    Ok(())
}
