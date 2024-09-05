#!/bin/bash

# read all env variables from wsl.env
source wsl.env
find ./examples -type f -name "*.rs" -exec basename {} \; | while read file; do
    echo "Running $file"
    cargo run --example ${file%.rs}
done

