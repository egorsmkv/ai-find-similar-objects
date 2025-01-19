# `Rust in Action`: embed your images into Qdrant and search similar objects using Rust and AI

## Overview

### Goal

Show how you can use Rust in the AI field to find similar objects in a vector database.

Also, join our Telegram group about Computer Vision: https://t.me/computer_vision_uk

### Algorithm

1. Upload a large file using [TUS protocol][4]
2. Detect objects using [YOLO 11][3]
3. [Embed][1] each image using [**clip-ViT-B-32**][2] and upload into **Qdrant**
4. Upload interested object and try to find similar ones

### Components

<details>
  <summary>Click here to see the list</summary>

Backend:

  - https://github.com/s3rius/rustus
  - https://github.com/qdrant/qdrant
  - https://github.com/qdrant/rust-client
  - https://github.com/Anush008/fastembed-rs
  - https://github.com/egorsmkv/yolo-inference
  - https://github.com/svenstaro/miniserve

Frontend:

  - https://github.com/gradio-app/gradio
  - https://github.com/qdrant/fastembed

Devtools:

  - https://github.com/astral-sh/uv
  - https://github.com/astral-sh/ruff
  - https://github.com/casey/just
  - https://github.com/goreleaser/goreleaser
  
</details>

## Install

### Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

rustup toolchain install nightly
```

### Install `uv`

```bash
curl -LsSf https://astral.sh/uv/install.sh | sh
```

### Install `just`

```bash
cargo install just
```

### Install `GoReleaser`

Basically, GoReleaser was written to release Go projects but they have added Rust support recently.

Read their docs about [installation process][5].

```bash
go install github.com/goreleaser/goreleaser/v2@latest
```

## Run

### Backend

#### Install & Run the TUS server

```bash
cargo install rustus

cd backend/rustus-server

cp .env.example .env.dev

just run
```

### Frontend

```bash
cd frontend/

# create virtual environment
uv venv --python 3.12

source .venv/bin/activate

# install all dependencies
just sync

# start Gradio app
just run
```

[1]: https://en.wikipedia.org/wiki/Word_embedding
[2]: https://huggingface.co/Qdrant/clip-ViT-B-32-vision
[3]: https://docs.ultralytics.com/models/yolo11/
[4]: https://tus.io/
[5]: https://goreleaser.com/install/
