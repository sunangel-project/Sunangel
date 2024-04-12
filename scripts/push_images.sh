#!/usr/bin/env bash

source ./scripts/services.sh

version="$1"
user="$2"
password="$3"

echo $password | docker login ghcr.io --username "$user" --password-stdin

tag_and_push() {
    service="$1"
    version="$2"

    docker tag "$service" "ghcr.io/sunangel-project/$service:$version"
    docker push "ghcr.io/sunangel-project/$service:$version"
}

for service in $services; do
    tag_and_push "$service" "$version"
    tag_and_push "$service" "latest"
done
