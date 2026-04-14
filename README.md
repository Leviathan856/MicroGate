# MicroGate
Lightweight async HTTP server framework suitable Linux embedded and IoT environments

MicroGate uses Rust programming language and is built on top of the Tokio async runtime. It is designed to be a lightweight and efficient HTTP server that can run on embedded Linux devices and IoT platforms. The server implements the HTTP/1.1 protocol according to RFC 9112.

MicroGate is enforced by WireFrame HTTP parser, which is implemented as a separate library. WireFrame documentation can be found in the corresponding repository in the README.md file.

TLS support is intentionally left out of the project, as it is typically handled at a different layer of the system and does not directly impact the HTTP protocol logic.

Routing is simple, based on method and path. The server supports chunked request bodies but does not support streaming responses or WebSockets.

Framework supports static storage. In addition it is designed to be easily extendable with additional features as needed.

## Implementation Overview

MicroGate is designed with safety, efficiency, and extensibility in mind using modern Rust best practices. The framework strictly avoids unsafe functions and potentially panicking code (like `unwrap` or `expect`), relying on explicit error propagation and safe defaults (`unwrap_or_else`, `unwrap_or_default`).

Key components include:
* **`Server`**: The core component that leverages `tokio::net::TcpListener` to asynchronously accept TCP connections. It maintains a reference-counted (`Arc`) router to efficiently share routes among concurrent handlers. Connection reading operates via a dynamically expandable buffer and interacts with the state-machine-controlled `WireFrame` HTTP parser. 
* **`Router`**: Implements simple, direct routing mapping `(HttpMethod, String)` pairs to boxed asynchronous handler functions. By using trait objects and `Pin<Box<dyn Future>>`, handlers provide immense flexibility.
* **`Response`**: Provides an intuitive builder pattern (`with_status`, `with_header`, `with_body`) to construct HTTP responses smoothly. Common failure states (e.g., standard 400 Bad Request, 404 Not Found, 500 Internal Server Error) are conveniently implemented as associated helper functions.
* **`fs::serve_file`**: Simplifies serving static assets from the filesystem, validating paths correctly to avoid directory traversal vulnerabilities, guessing MIME types, and directly mapping system errors to proper HTTP responses. 

## How to play / Use from another project

### 1. Add Dependencies
Add `microgate` to your `Cargo.toml`. Because `MicroGate` relies on the `WireFrame` HTTP parser, it will automatically fetch it from its GitHub repository, making this framework highly portable:

```toml
[dependencies]
microgate = { git = "https://github.com/andriy/MicroGate.git" }
tokio = { version = "1.0", features = ["full"] }
```

*(Note: Replace the GitHub URL with the actual remote repository URL when published).*

### 2. Implement your server application

Basic usage is highly straightforward thanks to `Router` mapping functionality.

```rust
use microgate::{Router, Server, Response, RequestContext};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    // 1. Initialize the Router
    let mut router = Router::new()
        // Registering a simple handler for GET method at root
        .get("/", |_ctx: RequestContext| async move {
            Response::new().with_body("Hello from MicroGate!")
        })
        // Registering a static file serving endpoint
        .get("/static", |ctx: RequestContext| async move {
            microgate::fs::serve_file("./public", &ctx.req.uri).await
        });

    // 2. Build and launch the server on loopback port 8080
    let server = Server::new("127.0.0.1:8080", router);
    println!("Server listening on http://127.0.0.1:8080");
    
    server.run().await
}
```

### CI/CD
This repository includes an automated GitHub Actions CI/CD workflow (`ci.yml`) validating compilation via `cargo build`/`test` and code quality enforcement utilizing `cargo fmt` and `cargo clippy`. Unsafe operations are prohibited, checked strictly through community-approved linting.
