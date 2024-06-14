FROM docker.io/rust:alpine AS builder

WORKDIR /app

# Install build dependencies
RUN apk update && apk add --no-cache \
    build-base \
    eudev-dev \
    pkgconfig

# copy the lockfiles first to allow docker to cache layer
COPY Cargo.toml Cargo.lock ./

COPY . .

RUN cargo build --release

# Stage 2: Runtime stage
FROM alpine:latest

# # Set the working directory
WORKDIR /app

# Copy the built binary from the build stage
COPY --from=builder /app/target/riscv32imc-unknown-none-elf/release/esp32c3-working .
