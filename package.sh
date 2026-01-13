#!/bin/bash
set -e

APP_NAME="local_lens"
VERSION="v0.1.1"
DIST_DIR="dist"
PACKAGE_NAME="${APP_NAME}_${VERSION}"
FULL_DIST_PATH="${DIST_DIR}/${PACKAGE_NAME}"

echo "📦 Packaging $APP_NAME $VERSION..."

# 1. Build Release Binary
echo "🛠️  Building release binary..."
cargo build --release

# 2. logical Clean / Create Dist Directory
if [ -d "$DIST_DIR" ]; then
    rm -rf "$DIST_DIR"
fi
mkdir -p "$FULL_DIST_PATH"

# 3. Copy Assets
echo "Cp Copying files..."
cp "target/release/$APP_NAME" "$FULL_DIST_PATH/"
cp "download-models.sh" "$FULL_DIST_PATH/"
cp "install.sh" "$FULL_DIST_PATH/"
cp "local_lens.desktop" "$FULL_DIST_PATH/"
cp "README.md" "$FULL_DIST_PATH/"

# Copy Models (Bundling them for offline install)
if [ -d "models" ]; then
    echo "📦 Bundling models..."
    mkdir -p "$FULL_DIST_PATH/models"
    cp -r models/* "$FULL_DIST_PATH/models/"
else
    echo "⚠️  WARNING: 'models' directory not found locally!"
    echo "   The package will be created without models."
    echo "   Users will need internet access to download them during install."
fi

# ensure scripts are executable
chmod +x "$FULL_DIST_PATH/download-models.sh"
chmod +x "$FULL_DIST_PATH/install.sh"

# 4. Create Archive
echo "🗜️  Creating archive..."
cd "$DIST_DIR"
tar -czvf "${PACKAGE_NAME}.tar.gz" "$PACKAGE_NAME"

echo "✅ Package created successfully:"
echo "   $DIST_DIR/${PACKAGE_NAME}.tar.gz"
