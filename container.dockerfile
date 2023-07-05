# this is not the dockerfile for the project, this is the dockerfile to
# build the container that fly-sccache uses.
#
# it is not supposed to be invoked directly, instead, use the
# ./build-container script

FROM rust:1-alpine AS builder

RUN apk add musl-dev openssl-dev pkgconfig

FROM alpine:latest AS container

RUN apk add openssl bash
ADD .build/cargo-target/x86_64-unknown-linux-musl/release/sccache /usr/local/bin
ADD .build/cargo-target/x86_64-unknown-linux-musl/release/fly-sccache-worker /usr/local/bin

CMD /usr/local/bin/fly-sccache-worker
