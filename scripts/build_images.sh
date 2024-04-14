#!/usr/bin/env bash

source ./scripts/services.sh

function build-image() {
    file_name="$1"
    image_name="$1"
    docker build . -f "Dockerfiles/$image_name" -t "$image_name"
}

build-image sunangel-rust-base

# new certificate for localhost
openssl req -x509 -newkey rsa:4096 -nodes -keyout key.pem -out cert.pem -days 365 -subj '/CN=localhost'

for service in $services; do
    build-image "$service" "$service"
done

if [ -n "$CLOUDSFTP_CERT_KEY" ] && [ -n "$CLOUDSFTP_CERT" ]; then
    echo "$CLOUDSFTP_CERT_KEY" > key.pem
    echo "$CLOUDSFTP_CERT" > cert.pem

    build-image "api" "api-cloudsftp"
fi
