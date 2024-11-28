#!/bin/sh

cd world
cargo build
cd ..

npm run build:wasm-release

npm install ./world/pkg
npm install -y

npm run load:wasm-assets

npm run build
