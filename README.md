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

## License

This project is licensed under the MIT License. See the LICENSE file for details.