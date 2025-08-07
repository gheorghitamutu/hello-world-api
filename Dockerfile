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

# Runtime stage - Use Distroless for maximum security and minimal size
FROM gcr.io/distroless/cc-debian12:latest

# Set the working directory
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/hello-world-api/target/release/hello-world-api /app/hello-world-api

# Note: Distroless doesn't support package installation, users, or shell scripts
# This provides maximum security but minimal debugging capabilities
# For debugging, use Dockerfile.debian-slim variant instead

# Expose the port
EXPOSE 8080/tcp

# Direct binary execution (no debug script support in distroless)
ENTRYPOINT ["/app/hello-world-api"]
