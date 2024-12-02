FROM rust:1.67-alpine

WORKDIR /usr/src
COPY . .

RUN cargo install --path .

CMD ["aoc2024"]
