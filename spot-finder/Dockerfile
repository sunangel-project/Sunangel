FROM rust:1.67.0

WORKDIR /usr/src/sunangel
COPY .. .
RUN cargo install --path spot-finder

CMD ["spot-finder"]
