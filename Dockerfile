FROM rust:alpine AS builder

WORKDIR /app

# Install dependencies
RUN apk update && apk add --no-cache \
    build-base \
    eudev-dev \
    pkgconfig

COPY . .

RUN cargo build --release

# Stage 2: Runtime stage
FROM alpine:latest

# # Set the working directory
WORKDIR /app

# Copy the built binary from the build stage
COPY --from=builder /app/target/riscv32imc-unknown-none-elf/release/esp32c3-working .
