# Use Rust official image to compile the binary
FROM rust:latest as builder

# Set working directory
WORKDIR /usr/src/app

# Copy manifests first to leverage Docker cache
COPY Cargo.toml ./

# Copy the .env file
COPY .env /usr/local/bin/.env

# Create a dummy src/main.rs file to optimize dependencies caching
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies first (caching)
RUN cargo build --release && rm -rf src target/release/deps/*

# Copy source files and build the final Rust binary
COPY . .
RUN cargo build --release

# Use Ubuntu 22.04 as the runtime image to avoid GLIBC issues
FROM ubuntu:22.04

# Install required system dependencies
RUN apt update && apt install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /usr/local/bin

# Copy the compiled Rust binary from the builder stage
COPY --from=builder /usr/src/app/target/release/RUSTFOUNDATION /usr/local/bin/RUSTFOUNDATION

# Expose port 3000
EXPOSE 3000

# Run the binary
CMD ["/usr/local/bin/RUSTFOUNDATION"]