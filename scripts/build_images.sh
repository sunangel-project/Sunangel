#!/usr/bin/env bash

push="false"
if [ "$1" = "--push" ]; then
    push="true"

    user="$2"
    password="$3"
    tag="latest"
fi

services="api spot-finder horizon-get horizon-compute sky-service"

function build-image() {
    image_name="$1"
    docker build . -f "Dockerfiles/$image_name" -t "$image_name:$tag"
}

build-image sunangel-rust-base

for service in $services; do
    build-image "${service}"
done

if [ "$push" = "false" ]; then
    exit 0
fi

docker login ghcr.io --username "$user" --password "$password"

for service in $services; do
    docker tag "$service" "ghcr.io/$user/$service:$tag"
    docker push "ghcr.io/$user/$service:$tag"
done
