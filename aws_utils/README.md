# AWS SDK Utilities Library

## TODO

### AWS Configuration

NOTE: The Utils crate requires AWS config files, with proper profiles, to be set up on the hosting app's system; it does not support environment configurations ATM.

see [AWS documentation](https://docs.aws.amazon.com/sdkref/latest/guide/file-format.html) for more information

### Logging

This utils library logs errors via the `log` crate, but is flexible and doesn't impose logging configurations on applications that use it. Application developers can choose their preferred logging framework and initialize it as needed.

#### Example Usage
```rust
 // Consumer app implementing the `env_logger` framework
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
