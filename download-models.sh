#!/bin/bash

# Directory to store models
MODELS_DIR="models"

# Create models directory if it doesn't exist
if [ ! -d "$MODELS_DIR" ]; then
    echo "Creating $MODELS_DIR directory..."
    mkdir -p "$MODELS_DIR"
fi

# Function to download file
download_file() {
    local url="$1"
    local output_path="$2"
    local filename=$(basename "$output_path")

    if [ -f "$output_path" ]; then
        echo "$filename already exists. Skipping."
        return
    fi

    echo "Downloading $filename..."
    
    if command -v wget &> /dev/null; then
        wget -q --show-progress -O "$output_path" "$url"
    elif command -v curl &> /dev/null; then
        curl -L -o "$output_path" "$url"
    else
        echo "Error: Neither wget nor curl is available. Please install one of them."
        exit 1
    fi

    if [ $? -eq 0 ]; then
        echo "Successfully downloaded $filename"
    else
        echo "Failed to download $filename"
        exit 1
    fi
}

echo "Starting model downloads..."

# 1. ResNet50 v2 (Image Classification)
download_file \
    "https://github.com/onnx/models/raw/main/validated/vision/classification/resnet/model/resnet50-v2-7.onnx" \
    "$MODELS_DIR/resnet50-v2-7.onnx"

# 2. ImageNet Labels
download_file \
    "https://raw.githubusercontent.com/anishathalye/imagenet-simple-labels/master/imagenet-simple-labels.json" \
    "$MODELS_DIR/imagenet-simple-labels.json"

# 3. OCR Detection Model (RapidOCR / PaddleOCR v3)
download_file \
    "https://github.com/Kazuhito00/PaddleOCRv3-ONNX-Sample/raw/main/ppocr_onnx/model/det_model/ch_PP-OCRv3_det_infer.onnx" \
    "$MODELS_DIR/det_model.onnx"

# 4. OCR Recognition Model (RapidOCR / PaddleOCR v3 - English)
download_file \
    "https://github.com/Kazuhito00/PaddleOCRv3-ONNX-Sample/raw/main/ppocr_onnx/model/rec_model/en_PP-OCRv3_rec_infer.onnx" \
    "$MODELS_DIR/rec_model.onnx"

# 5. OCR Keys/Dictionary (English)
# Remove old chinese keys if present to avoid confusion
if [ -f "$MODELS_DIR/ppocr_keys_v1.txt" ]; then
    rm "$MODELS_DIR/ppocr_keys_v1.txt"
fi

download_file \
    "https://raw.githubusercontent.com/PaddlePaddle/PaddleOCR/main/ppocr/utils/en_dict.txt" \
    "$MODELS_DIR/en_dict.txt"

echo "All models downloaded successfully to '$MODELS_DIR/'."
