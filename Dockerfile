# syntax=docker/dockerfile:1

FROM rust:1.73-slim

WORKDIR /stsearch
COPY . .

RUN cargo build --all-features --release
