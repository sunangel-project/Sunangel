#!/usr/bin/env bash

source ./scripts/services.sh

function build-image() {
    image_name="$1"
    docker build . -f "Dockerfiles/$image_name" -t "$image_name"
}

build-image sunangel-rust-base

for service in $services; do
    build-image "${service}"
done
