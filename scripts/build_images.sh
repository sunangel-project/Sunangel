#!/usr/bin/env bash

source ./scripts/services.sh

function build-image() {
    file_name="$1"
    image_name="$2"
    docker build . -f "Dockerfiles/$file_name" -t "$image_name"
}

build-image "rust-build" "rust-build"
build-image "rust-run" "rust-run"

for service in $services; do
    build-image "$service" "$service"
done
