# Rust-Axum Todo List API
This is a simple API based on [this guide](https://codevoweb.com/create-a-simple-api-in-rust-using-the-axum-framework/)

The idea is to practice Rust and learn good practices for API design using the Axum framework.

## List of dependencies

```toml
[dependencies]
axum = "0.6.20"
chrono = { version = "0.4.31", features = ["serde"] }
serde = { version = "1.0.188", features = ["derive"] }
serde_json = "1.0.107"
tokio = { version = "1.32.0", features = ["full"] }
tower-http = { version = "0.4.4", features = ["cors"] }
uuid = { version = "1.4.1", features = ["v4", "serde"] }
```