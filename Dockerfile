FROM rust:1.67 as builder
WORKDIR /usr/src/request-reflector
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim

EXPOSE 3000
COPY --from=builder /usr/local/cargo/bin/request-reflector /usr/local/bin/request-reflector
CMD ["request-reflector"]