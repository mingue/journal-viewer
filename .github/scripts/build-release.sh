#! /bin/bash

set -e

npm install
pkgver=${GITHUB_REF_NAME:1}
sed -i "s/0.0.1/$pkgver/g" ./src-tauri/tauri.conf.json
npm run tauri build
