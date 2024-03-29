# syntax=docker/dockerfile:1

FROM rust:1.64-slim

WORKDIR /stsearch
COPY . .

RUN cargo build --release
