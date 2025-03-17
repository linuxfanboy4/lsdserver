# LSD Server

LSD Server is a lightweight, dependency-free HTTP server library written in Rust. It is designed to be simple, fast, and easy to use, making it an ideal choice for serving static files or building small web applications. The library is minimalistic, with no external dependencies, ensuring that your project remains lightweight and efficient.

## Features

- **Lightweight**: No external dependencies, ensuring minimal overhead.
- **Simple API**: Easy-to-use functions for starting a server and handling connections.
- **Static File Serving**: Serve HTML, CSS, JavaScript, and other static files with automatic content type detection.
- **Customizable**: Easily extendable to handle more complex use cases.

## Installation

To add LSD Server to your project, use Cargo:

```bash
cargo add lsdserver
```

This will add the latest version of LSD Server to your `Cargo.toml` file.

## Usage

### Basic Example

Here's a simple example of how to use LSD Server to serve static files:

```rust
use lsdserver::{start_server, handle_connection};

fn main() {
    // Start the server on localhost:8080
    start_server("127.0.0.1:8080");
}
```

This will start an HTTP server on `127.0.0.1:8080`. By default, the server will serve files from the current directory. For example, if you have an `index.html` file in the root directory, it will be served when you navigate to `http://127.0.0.1:8080/`.

### Handling Connections

The `handle_connection` function is responsible for processing incoming HTTP requests. It reads the request, determines the requested file, and serves it with the appropriate content type. If the file does not exist, it returns a 404 Not Found response.

### Starting the Server

The `start_server` function binds the server to the specified address and starts listening for incoming connections. It prints a message to the console indicating that the server has started.

## Customization

LSD Server is designed to be simple, but it can be easily extended to handle more complex use cases. For example, you can modify the `handle_connection` function to implement custom routing, middleware, or other advanced features.

## License

LSD Server is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details. 
