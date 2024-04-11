#!/usr/bin/env bash

source ./scripts/services.sh

version="$1"
user="$2"
password="$3"

echo $password | docker login ghcr.io --username "$user" --password-stdin

for service in $services; do
    docker tag "$service" "ghcr.io/sunangel-project/$service"
    docker push "ghcr.io/sunangel-project/$service:$version"
    docker push "ghcr.io/sunangel-project/$service:latest"
done
