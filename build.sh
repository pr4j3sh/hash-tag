#!/bin/bash

wasm-pack build --target nodejs --release
rm -rf ./pkg/.gitignore
nvim ./pkg/package.json
