import random
import sys
from importlib.metadata import version

import gradio as gr

concurrency_limit = 5

description_head = """
# Rust in Action

Embed your images into Qdrant and search similar objects using Rust and AI
""".strip()

result_value = """
Similar objects will appear here.
""".strip()

tech_env = f"""
#### Environment
- Python: {sys.version}
""".strip()

tech_libraries = f"""
#### Libraries
- requests: {version("requests")}
- gradio: {version("gradio")}
""".strip()


def find_objects(object_path, progress=gr.Progress()):
    if not object_path:
        raise gr.Error("Please upload an object file.")

    progress(0, desc="Searching...")

    gr.Success("Found 0 objects!", duration=2)

    images = [
        (
            random.choice(
                [
                    "http://localhost:8099/test/flash-1.jpeg",
                    "http://localhost:8099/test/flash-2.jpeg",
                    "http://localhost:8099/test/flash-3.jpeg",
                    "http://localhost:8099/test/flash-1.jpeg",
                    "http://localhost:8099/test/flash-2.jpeg",
                    "http://localhost:8099/test/flash-3.jpeg",
                    "http://localhost:8099/test/flash-1.jpeg",
                    "http://localhost:8099/test/flash-2.jpeg",
                    "http://localhost:8099/test/flash-3.jpeg",
                    "http://localhost:8099/test/flash-1.jpeg",
                    "http://localhost:8099/test/flash-2.jpeg",
                    "http://localhost:8099/test/flash-3.jpeg",
                    "http://localhost:8099/test/flash-1.jpeg",
                    "http://localhost:8099/test/flash-2.jpeg",
                    "http://localhost:8099/test/flash-3.jpeg",
                ]
            ),
            f"label {i}",
        )
        for i in range(9)
    ]

    return images


def upload_image(image_path, progress=gr.Progress()):
    if not image_path:
        raise gr.Error("Please upload an image file.")

    progress(0, desc="Uploading...")

    gr.Success("Successfully uploaded!", duration=2)

    return "Done"


demo = gr.Blocks(
    title="Rust in Action",
    analytics_enabled=False,
    theme=gr.themes.Base(),
)

with demo:
    gr.Markdown(description_head)

    gr.Markdown("## Find objects")

    with gr.Row():
        object_file = gr.Image(label="Your object file", type="filepath")
        outputs = gr.Gallery(
            label="Results",
            show_label=False,
            elem_id="gallery",
            columns=[3],
            rows=[1],
            object_fit="contain",
            height="auto",
        )

    gr.Button("Find").click(
        find_objects,
        concurrency_limit=concurrency_limit,
        inputs=object_file,
        outputs=outputs,
    )

    gr.Markdown("## Upload images for emdedding")

    with gr.Row():
        image_file = gr.Image(label="Your image file", type="filepath")
        outputs = gr.Markdown(
            label="Result",
            value="Status will appear here.",
        )

    gr.Button("Upload").click(
        upload_image,
        concurrency_limit=concurrency_limit,
        inputs=image_file,
        outputs=outputs,
    )

    gr.Markdown("### Gradio app uses:")
    gr.Markdown(tech_env)
    gr.Markdown(tech_libraries)

if __name__ == "__main__":
    demo.queue()
    demo.launch()
