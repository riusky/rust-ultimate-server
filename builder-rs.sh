#!/bin/bash
set -euo pipefail

OUT_DIR="rs-builder/rs-builder-target"

rm -rf "$OUT_DIR"
mkdir -p "$OUT_DIR"

# 构建 Docker 镜像
echo "正在构建 rs-builder 镜像..."
docker build -t rs-builder -f dockerfile .

# 运行容器并复制编译产物
echo "正在编译 Rust 项目..."
docker run --rm \
  -v "$(pwd)/$OUT_DIR:/output" \
  rs-builder

echo "编译完成，输出目录: $OUT_DIR"