#! /bin/bash

set -e

npm install
npm run build-only
cd src-tauri
cargo clippy
cargo build
