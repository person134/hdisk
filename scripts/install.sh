#!/bin/bash
set -e

BINARY="hdisk"
INSTALL_DIR="/usr/local/bin"

if [ "$1" = "--uninstall" ]; then
    echo "Removing $BINARY from $INSTALL_DIR..."
    rm -f "$INSTALL_DIR/$BINARY"
    echo "Done."
    exit 0
fi

echo "Building $BINARY..."
cargo build --release

echo "Installing $BINARY to $INSTALL_DIR..."
cp "target/release/$BINARY" "$INSTALL_DIR/$BINARY"
chmod +x "$INSTALL_DIR/$BINARY"
echo "Installed. Run '$BINARY' to start."
