#!/usr/bin/env bash
set -euo pipefail

APP_NAME="ly-translate"
VERSION=$(node -p "require('./package.json').version")
WIN_TARGET="x86_64-pc-windows-msvc"
OUT_DIR="dist-win"

echo "=== Building ${APP_NAME} v${VERSION} for Windows ==="

# Build frontend
echo "[1/3] Building frontend..."
npm ci && npm run build

# Build via Docker
echo "[2/3] Cross-compiling Rust binary via Docker..."
docker build -f Dockerfile.windows -t ${APP_NAME}-win-builder .

# Extract binary from container
echo "[3/3] Extracting artifacts..."
CONTAINER_ID=$(docker create ${APP_NAME}-win-builder)
rm -rf ${OUT_DIR} && mkdir -p ${OUT_DIR}

docker cp ${CONTAINER_ID}:/app/src-tauri/target/${WIN_TARGET}/release/${APP_NAME}.exe ${OUT_DIR}/
cp -r dist ${OUT_DIR}/frontend-dist

# Copy the icon
cp src-tauri/icons/icon.ico ${OUT_DIR}/ 2>/dev/null || true

# Create zip
cd ${OUT_DIR} && zip -r "../${APP_NAME}-v${VERSION}-windows-x64.zip" . && cd ..
rm -rf ${OUT_DIR}
docker rm ${CONTAINER_ID} > /dev/null

echo ""
echo "=== Done ==="
echo "Output: ${APP_NAME}-v${VERSION}-windows-x64.zip"
echo ""
echo "Contents:"
echo "  ${APP_NAME}.exe    - main executable"
echo "  frontend-dist/     - web assets (keep alongside exe)"
echo "  icon.ico           - app icon"
