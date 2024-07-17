#!/bin/bash
rustup target install wasm32-unknown-unknown

cargo install wasm-server-runner 

cargo install wasm-bindgen-cli 