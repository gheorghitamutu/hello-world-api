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

# Runtime stage - Use Red Hat UBI for better OpenShift compatibility
FROM registry.access.redhat.com/ubi9/ubi-minimal:latest

# Install CA certificates and create non-root user
RUN microdnf update -y && \
    microdnf install -y ca-certificates && \
    microdnf clean all && \
    # Create a non-root user for OpenShift security
    useradd -r -u 1001 -g root -s /sbin/nologin \
            -c "Default Application User" appuser

# Set the working directory
WORKDIR /app

# Copy the compiled binary from the builder stage
COPY --from=builder /usr/src/hello-world-api/target/release/hello-world-api /app/hello-world-api

# Set proper ownership and permissions for OpenShift
RUN chown -R 1001:0 /app && \
    chmod -R g+rw /app && \
    chmod +x /app/hello-world-api

# Switch to non-root user
USER 1001

# Expose the port (OpenShift will dynamically assign ports)
EXPOSE 8080

# Use ENTRYPOINT for better signal handling in Kubernetes
ENTRYPOINT ["/app/hello-world-api"]
