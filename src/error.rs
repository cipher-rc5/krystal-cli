// file: src/error.rs
// description: Error types and handling for the Krystal API client, providing comprehensive
//             error categorization with retryability detection and user-friendly messages
// docs_reference: https://docs.rs/thiserror/latest/thiserror/

use reqwest::Error as ReqwestError;
use thiserror::Error;

/// Custom error types for better error handling and debugging.
#[derive(Error, Debug)]
pub enum KrystalApiError {
    /// HTTP request failed
    #[error("HTTP request failed: {0}")]
    RequestError(#[from] ReqwestError),

    /// API returned an error response
    #[error("API returned error: {status} - {message}")]
    ApiError { status: u16, message: String },

    /// Authentication failed
    #[error("Authentication failed: Missing or invalid API key")]
    AuthError,

    /// Payment required - no credit left
    #[error("Payment required: No credit left")]
    PaymentRequired,

    /// Invalid parameters provided
    #[error("Invalid parameters: {0}")]
    InvalidParams(String),

    /// URL parsing error
    #[error("URL parsing error: {0}")]
    UrlError(#[from] url::ParseError),

    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    JsonError(#[from] serde_json::Error),

    /// Environment variable error
    #[error("Environment variable error: {0}")]
    EnvError(#[from] std::env::VarError),
}

/// Result type alias for convenience
pub type Result<T> = std::result::Result<T, KrystalApiError>;

impl KrystalApiError {
    /// Check if error is retryable
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            Self::RequestError(_)
                | Self::ApiError {
                    status: 500..=599,
                    ..
                }
        )
    }

    /// Check if error is related to authentication
    pub fn is_auth_error(&self) -> bool {
        matches!(self, Self::AuthError)
    }

    /// Check if error requires payment
    pub fn requires_payment(&self) -> bool {
        matches!(self, Self::PaymentRequired)
    }

    /// Get user-friendly error message with suggested actions
    pub fn user_message(&self) -> String {
        match self {
            Self::AuthError => {
                "Authentication failed. Please check your API key is correct and has proper permissions.".to_string()
            }
            Self::PaymentRequired => {
                "Your account has no remaining credits. Please top up your balance to continue.".to_string()
            }
            Self::RequestError(e) if e.is_timeout() => {
                "Request timed out. Please try again or check your internet connection.".to_string()
            }
            Self::RequestError(e) if e.is_connect() => {
                "Could not connect to the API. Please check your internet connection.".to_string()
            }
            Self::InvalidParams(msg) => {
                format!("Invalid request parameters: {}", msg)
            }
            _ => self.to_string(),
        }
    }
}
