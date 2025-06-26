#!/usr/bin/env bash
set -e

BASE_DIR="$(cd "$(dirname "$0")/.." && pwd)"
DATE="$(date +%F)"

cp "$BASE_DIR/buildroot/.config" "$BASE_DIR/configs/config-$DATE"

echo ".config archived as configs/config-$DATE"

