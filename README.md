# Hello World API

This is a simple Rust-based API that responds with "Hello, World!" to incoming HTTP requests.

## Project Structure

```
hello-world-api
├── src
│   └── main.rs       # Entry point of the application
├── Cargo.toml        # Rust project configuration
├── Dockerfile         # Dockerfile for building the application
└── README.md         # Project documentation
```

## Getting Started

### Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)
- Docker (for containerization)

### Building the Project

To build the project, navigate to the project directory and run:

```bash
cargo build
```

### Running the Project

To run the project locally, use the following command:

```bash
cargo run
```

The API will be available at `http://localhost:8000`.

### Using Docker

To build the Docker image, run:

```bash
docker build -t hello-world-api .
```

To run the Docker container, use:

```bash
docker run -p 8000:8000 hello-world-api
```

The API will be accessible at `http://localhost:8000`.

## Debugging

This project includes comprehensive debugging tools for Kubernetes/OpenShift environments. See [DEBUG.md](DEBUG.md) for detailed debugging instructions.

### Quick Debug Setup

```bash
# Deploy in debug mode (container sleeps for manual debugging)
oc apply -f k8s/deployment-debug.yaml

# Exec into the pod
POD_NAME=$(oc get pods -l app=hello-world-api-debug -o jsonpath='{.items[0].metadata.name}')
oc exec -it $POD_NAME -- /bin/sh

# Use the debug script
./debug.sh run    # Run with monitoring
./debug.sh info   # Show system info
./debug.sh test   # Test API endpoints
```

The container includes debugging tools like `ps`, `top`, `curl`, `strace`, `lsof`, and `htop`.

## License

This project is licensed under the MIT License. See the LICENSE file for details.