# AWS Utilities

## TODO

### In Development

- see the utils crate [README.md](./aws_utils/README.md) for more details.

### Example Usage

Shows a consuming application implementing logging with`env_logger` framework to capture log messages from the utils crate

```rust
 use aws_utils::create_client;
 use env_logger;
 use log::info;

 #[tokio::main]
 async fn main() {
     env_logger::init();
     let client = create_client().await.unwrap();
     info!("Client created: {:?}", client);
 }
```
