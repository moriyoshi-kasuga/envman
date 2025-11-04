pub trait EnvMan: Sized {
    /// Load environment variables
    fn load_from_env() -> Result<Self, EnvManError>;
}

/// Error type for [`EnvMan`]
#[derive(thiserror::Error, Debug)]
pub enum EnvManError {
    /// Failed to read environment variable
    #[error("failed to read environment variable '{key}'")]
    NotFound { key: &'static str },

    /// Failed to parse environment variable
    #[error("failed to parse environment variable '{key}' with value '{value}' (expected type: {expected_type})")]
    Parse {
        key: &'static str,
        value: String,
        expected_type: &'static str,
        #[source]
        source: Box<dyn std::error::Error>,
    },

    /// Failed validation for environment variable
    #[error("validation failed for environment variable '{key}' with value '{value}': {message}")]
    Validation {
        key: &'static str,
        value: String,
        message: String,
    },

    /// Multiple errors occurred while loading environment variables
    #[error("multiple errors occurred while loading environment variables:\n{}", format_errors(.0))]
    Multiple(Vec<EnvManError>),
}

fn format_errors(errors: &[EnvManError]) -> String {
    errors
        .iter()
        .enumerate()
        .map(|(i, e)| format!("  {}. {}", i + 1, e))
        .collect::<Vec<_>>()
        .join("\n")
}
