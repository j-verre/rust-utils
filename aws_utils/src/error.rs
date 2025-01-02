use std::fmt;

#[allow(dead_code, reason="in development")]
#[derive(Debug)]
pub enum UtilsError {
    AwsConfigurationRegionMissing,
    General(ErrorMessage),
}

impl std::error::Error for UtilsError {}

impl fmt::Display for UtilsError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Utils Error: {}", self)
    }
}

#[derive(Debug)]
pub struct ErrorMessage(pub String);

impl fmt::Display for ErrorMessage {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ErrorMessage({})", self.0)
    }
}
