#!/bin/bash

set -e

which wasm-pack || cargo install --version 0.10.1 wasm-pack

# pack for browser
wasm-pack build --release --target=web --out-name=mini-goldilocks-web --out-dir=web-dist
# pack for node.js
wasm-pack build --release --target=nodejs --out-name=mini-goldilocks-node --out-dir=node-dist

# Merge dist folders
mv web-dist/* dist/
mv node-dist/* dist/
rm -rf web-dist node-dist
rm dist/package.json dist/.gitignore

if [ "$CI" == "1" ]; then
    exit 0
fi
