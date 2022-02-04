# Build frontend
FROM node:17-alpine as frontend

WORKDIR /frontend

ENV PATH /frontend/node_modules/.bin:$PATH

COPY ./frontend ./
RUN yarn install --ignore-engines
RUN yarn build

# Compile server
FROM ekidd/rust-musl-builder:nightly-2021-12-23 AS builder

ADD --chown=rust:rust . ./

RUN cargo build --release

# Final image
FROM alpine:latest
RUN apk update && apk add bash

WORKDIR /

COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/linqs ./
COPY --from=frontend /frontend/dist ./frontend/dist

CMD ["/linqs"]
