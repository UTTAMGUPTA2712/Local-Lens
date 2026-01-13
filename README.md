# Local Lens

**Local Lens** is a privacy-focused, AI-powered image tagging and organization tool written in Rust. It runs entirely locally on your machine, ensuring your photos are never uploaded to the cloud.

## Features

-   **Deep Learning Tagging**: Uses a ResNet50 ONNX model to automatically detect objects in images.
-   **OCR Support**: Extracts text from images and adds it as searchable tags.
-   **Metadata Embedding**: Writes tags directly into image metadata (EXIF/XMP) using `exiftool`, making them searchable by your OS file manager.
-   **SQLite Database**: maintains a local index for fast searching within the app.
-   **Privacy**: No internet connection required for tagging.
-   **Global Renaming**: Rename tags across your entire library.

## Prerequisites

-   **Rust**: Stable toolchain (install via [rustup](https://rustup.rs/)).
-   **ExifTool**: Required for writing metadata to files.
    -   Ubuntu/Debian: `sudo apt install libimage-exiftool-perl`
    -   macOS: `brew install exiftool`
    -   Windows: Download executable and add to PATH.

## Setup

1.  **Clone the repository**:
    ```bash
    git clone https://github.com/uttamgupta2712/local_lens.git
    cd local_lens
    ```

2.  **Download Models**:
    Run the included script to automatically download all required models:
    ```bash
    ./download-models.sh
    ```
    This will fetch:
    -   `resnet50-v2-7.onnx` (ResNet50 v2)
    -   `imagenet-simple-labels.json`
    -   `det_model.onnx` (OCR Detection)
    -   `rec_model.onnx` (OCR Recognition)
    -   `ppocr_keys_v1.txt` (OCR Dictionary)
    
    *(Note: You need `wget` or `curl` installed)*

3.  **Run the Application**:
    ```bash
    cargo run --release
    ```

## Usage

1.  **Tagging**:
    -   Enter the folder path containing your images in the sidebar.
    -   Click **Tag Images**.
    -   The app will scan, generate tags, and write them to the files.

2.  **Search**:
    -   Enter keywords in the "Search query" box to filter processed images.

3.  **Manage Tags**:
    -   Use the "Rename Globally" section to fix typos or change tag names across all indexed images.

## Architecture

-   **Frontend**: `egui` (Immediate Mode GUI)
-   **Inference**: `ort` (ONNX Runtime bindings)
-   **Database**: `rusqlite`
