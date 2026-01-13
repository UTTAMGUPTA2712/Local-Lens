#!/bin/bash
set -e

INSTALL_DATA="$HOME/.local/share/local_lens/models"
INSTALL_BIN="$HOME/.local/bin"
INSTALL_SHARE="$HOME/.local/share/applications"
APP_NAME="local_lens"

echo "Installing $APP_NAME..."

# Create directories
mkdir -p "$INSTALL_BIN"
mkdir -p "$INSTALL_SHARE"
mkdir -p "$INSTALL_DATA"

# Copy executable
echo "Copying binary to $INSTALL_BIN..."
cp "$APP_NAME" "$INSTALL_BIN/"
chmod +x "$INSTALL_BIN/$APP_NAME"

# Copy desktop file
echo "Copying desktop entry to $INSTALL_SHARE..."
# Update Exec path in desktop file to absolute path
sed "s|Exec=local_lens|Exec=$INSTALL_BIN/$APP_NAME|g" local_lens.desktop > "$INSTALL_SHARE/$APP_NAME.desktop"

# Download models if not present
if [ ! -d "models" ]; then
    echo "Running model download..."
    ./download-models.sh
fi

# Copy models
echo "Copying models to $INSTALL_DATA..."
cp -r models/* "$INSTALL_DATA/"

echo "✅ Installation complete!"
echo "You can now launch 'Local Lens' from your application menu."
echo "Models installed to: $INSTALL_DATA"
