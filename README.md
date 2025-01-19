# `Rust in Action`: embed your images into Qdrant and search similar objects using Rust and AI

## Overview

### Goal

Show how you can use Rust in the AI field to find similar objects in a vector database.

### Algorithm

1. Upload a large file
2. Detect objects using YOLO 11
3. Embed each image using **clip-ViT-B-32** and upload into **Qdrant**
4. Upload interested object and try to find similar ones

## Components

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

## Run

### Backend

...

### Frontend

...


[1]: https://en.wikipedia.org/wiki/Word_embedding
