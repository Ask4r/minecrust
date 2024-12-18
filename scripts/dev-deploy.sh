#!/bin/sh

cd world
cargo install wasm-pack
cargo build
cd ..

npm run build:wasm

npm install ./world/pkg
npm install -y

npm run load:wasm-assets

npm run build
