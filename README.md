# Local Lens

**Local Lens** is a privacy-focused, AI-powered image tagging and organization tool written in Rust. It runs entirely locally on your machine, ensuring your photos are never uploaded to the cloud.

## Features

-   **Deep Learning Tagging**: Uses a ResNet50 ONNX model to automatically detect objects in images.
-   **OCR Support**: Extracts text from images and adds it as searchable tags.
-   **Metadata Embedding**: Writes tags directly into image metadata (EXIF/XMP) using `exiftool`, making them searchable by your OS file manager.
-   **SQLite Database**: maintains a local index for fast searching within the app.
-   **Privacy**: No internet connection required for tagging.
-   **Global Renaming**: Rename tags across your entire library.

## One-Click Downloads

**"I just want to use the app!"**

Go to the [**Releases Page**](https://github.com/uttamgupta2712/local_lens/releases) and download the file for your system:

-   **Windows**: Download `local_lens-windows.zip`. Extract it and run `local_lens.exe`.
-   **Linux**: Download `local_lens-linux.tar.gz`. Extract it and run `./install.sh`.
-   **macOS**: Download `local_lens-macos.tar.gz`. Extract it and run `./local_lens`.

*(Note: On first run, models will process or download automatically if using the scripts above)*

## Prerequisites (for building from source)

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

## Distribution

To package the application for sharing:

1.  Run the packaging script:
    ```bash
    ./package.sh
    ```
    This will build the release binary and create a `dist/local_lens_v0.1.0.tar.gz` archive.

2.  Share the `.tar.gz` file. The recipient should:
    -   Extract the archive: `tar -xzvf local_lens_dist.tar.gz`
    -   Open the folder: `cd local_lens_v0.1.0`
    -   **Option A (Run once)**:
        -   Run `./download-models.sh`
        -   Run `./local_lens`
    -   **Option B (Install as App)**:
        -   Run `./install.sh`
        -   This will download models (if missing), install them to `~/.local/share/local_lens/models`, copy the app to `~/.local/bin`, and add it to the application menu.
        -   You can then launch "Local Lens" from your system launcher.
