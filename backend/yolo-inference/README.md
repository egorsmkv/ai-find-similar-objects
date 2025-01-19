# YOLO inference

## Download the model

```
wget "https://huggingface.co/pan93412/yolo-v11-onnx/resolve/main/yolo11x.onnx"
```

## Run

### Dev

```
RUST_LOG=debug cargo run -- \
  --images-file images.txt \
  --yolo-model yolo11x.onnx \
  --probability-threshold 0.5 \
  --iou-threshold 0.7
```

### Prod

```
cargo build --release

cp target/release/yolo-inference .

./yolo-inference \
  --images-file images.txt \
  --yolo-model yolo11x.onnx \
  --probability-threshold 0.5 \
  --iou-threshold 0.7
```
