#![doc = include_str!("../README.md")]

mod error;

use aws_config::{
    meta::region::RegionProviderChain, profile::ProfileFileRegionProvider, BehaviorVersion,
};
use aws_sdk_s3::Client;
use error::UtilsError;
use log::{error, info};

const DEFAULT_REGION: &str = "us-west-2";

/// Initializes an AWS client.
///
/// Uses a given AWS profile set up in host system config file, falling back to the default profile if None provided.
///
/// IMPORTANT: Requires AWS config files and profiles on the hosting app's system
/// - For more details, see the README.md file.
pub async fn create_client(profile_name: Option<&str>) -> Result<Client, UtilsError> {
    match configure(profile_name).await {
        Ok(config) => {
            info!("Client created successfully");
            Ok(Client::new(&config))
        }
        Err(error) => {
            error!("Error: Utils failed to create client: {:?}", error);
            Err(error)
        }
    }
}

async fn configure(profile_name: Option<&str>) -> Result<aws_config::SdkConfig, UtilsError> {
    // Use the given profile name, falling back to the default profile if None provided
    let profile_provider = ProfileFileRegionProvider::builder()
        .profile_name(profile_name.unwrap_or("default"))
        .build();

    let region_provider = RegionProviderChain::first_try(profile_provider)
        .or_default_provider()
        // TODO: currently, fallback ALWAYS provides a region, it does not error on missing region
        .or_else(DEFAULT_REGION);

    if let Some(region) = region_provider.region().await {
        info!("{region:#?}");
    } else {
        error!("Missing region");
        return Err(UtilsError::AwsConfigurationRegionMissing);
    }

    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(region_provider)
        .load()
        .await;

    Ok(config)
}

#[cfg(test)]
mod tests {
    // NOTE: unit tests can be fickle since they require local AWS configuration files with profiles to be set up properly
    use super::*;
    use env_logger;
    use std::sync::Once;
    use tokio;

    static INIT: Once = Once::new();

    fn setup() {
        INIT.call_once(|| {
            // logger can only be initialized one time within a test suite
            env_logger::init();
        });
    }

    #[tokio::test]
    async fn test_create_client_none_profile() {
        setup();
        // Passing no profile name should fallback to defaults
        let result = create_client(None).await;
        assert!(result.is_ok(), "Expected Ok, got {:?}", result);
    }

    #[tokio::test]
    async fn test_create_client_bad_profile() {
        setup();
        // Passing a bad profile name should fallback to defaults
        let result = configure(Some("FAKE_PROFILE")).await;
        assert!(result.is_ok(), "Expected Ok, got {:?}", result);
    }

    #[tokio::test]
    async fn test_configure_none_success() {
        setup();
        let result = configure(None).await;
        assert!(result.is_ok(), "Expected Ok, got {:?}", result);
    }
}
