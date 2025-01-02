use env_logger;
use log::{error, info};

#[allow(dead_code, reason = "development purposes")]
const AWS_PROFILE_DEV: &str = "rust-aws-utils";
const AWS_PROFILE_DEFAULT: &str = "default";

#[tokio::main]
async fn main() {
    env_logger::init();
    let client = aws_utils::create_client(Some(AWS_PROFILE_DEFAULT)).await;
    if let Err(error) = client {
        error!("Application failed to create AWS client {error}");
        std::process::exit(-99)
    }
    info!("Done.");
}
