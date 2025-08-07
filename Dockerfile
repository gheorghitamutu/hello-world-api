# Build stage
FROM rust:latest as builder

WORKDIR /usr/src/hello-world-api

# Copy dependency files first for better caching
COPY Cargo.toml Cargo.lock ./

# Create a dummy main.rs to build dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs

# Build dependencies (this layer will be cached unless Cargo.toml changes)
RUN cargo build --release && rm src/main.rs

# Copy source code and build the application
COPY src ./src
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim

# Install CA certificates for HTTPS requests (if needed)
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/hello-world-api/target/release/hello-world-api /app/hello-world-api

# Expose the port
# EXPOSE 8080

# Run the binary
CMD ["./hello-world-api"]
