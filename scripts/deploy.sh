#!/bin/sh

cd world
rustup target add wasm32-unknown-unknown --toolchain nightly
cargo install wasm-pack
cargo build
cd ..

npm run build:wasm-release

npm install ./world/pkg
npm install -y

npm run load:wasm-assets

npm run build
