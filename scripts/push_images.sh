#!/usr/bin/env sh

source ./scripts/services.sh

version="$1"
user="$2"
password="$3"

echo $password | docker login ghcr.io --username "$user" --password-stdin

for service in $services; do
    docker push "ghcr.io/$user/$service:$version"
    docker push "ghcr.io/$user/$service:latest"
done
