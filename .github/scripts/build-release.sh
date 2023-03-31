#! /bin/bash

set -e

sudo ln -s /lib/x86_64-linux-gnu/libsystemd.so.0 /lib/x86_64-linux-gnu/libsystemd.so
npm install
pkgver=${GITHUB_REF_NAME:1}
sed -i "s/0.0.1/$pkgver/g" ./src-tauri/tauri.conf.json
npm run tauri build
