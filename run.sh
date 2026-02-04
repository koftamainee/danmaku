#!/usr/bin/env bash
set -e
cd "$(dirname "$0")"
meson compile -C build
./build/danmaku
