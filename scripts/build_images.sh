#!/usr/bin/env bash

TAG="latest"

SERVICES="api spot-finder horizon-get horizon-compute sky-service"

function build-image() {
    IMAGE_NAME="$1"
    docker build . -f "Dockerfiles/${IMAGE_NAME}" -t "${IMAGE_NAME}:${TAG}"
}

build-image sunangel-rust-base

for service in ${SERVICES}; do
    build-image "${service}"
done
