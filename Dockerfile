FROM alpine:latest AS builder

WORKDIR /app

# Install dependencies
RUN apk update && apk add --no-cache \
    build-base \
    libusb \
    eudev-dev \
    git \
    pkgconfig \
    curl

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Set environment variables for Rust
ENV PATH="/root/.cargo/bin:${PATH}"

COPY . .

RUN cargo build --release

# Stage 2: Runtime stage
FROM alpine:latest

# # Set the working directory
WORKDIR /app

# Copy the built binary from the build stage
COPY --from=builder /app/target/riscv32imc-unknown-none-elf/release/esp32c3-working .

