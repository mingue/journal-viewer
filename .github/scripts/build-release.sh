#! /bin/bash

set -e

sudo ln -s /lib/x86_64-linux-gnu/libsystemd.so.0 /lib/x86_64-linux-gnu/libsystemd.so
npm install
npm run tauri build
