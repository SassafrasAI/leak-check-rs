use thiserror::Error;

#[derive(Error, Debug)]
pub enum LeakCheckError {
    #[error("Missing API key")]
    MissingApiKey,

    #[error("Invalid API key")]
    InvalidApiKey,

    #[error("Invalid query type")]
    InvalidType,

    #[error("Invalid email")]
    InvalidEmail,

    #[error("Invalid query")]
    InvalidQuery,

    #[error("Invalid domain")]
    InvalidDomain,

    #[error("Query too short (minimum 3 characters)")]
    QueryTooShort,

    #[error("Invalid characters in query")]
    InvalidCharacters,

    #[error("Rate limited: too many requests")]
    RateLimited,

    #[error("Active plan required")]
    PlanRequired,

    #[error("Quota limit reached")]
    QuotaReached,

    #[error("Could not determine search type automatically")]
    UnknownType,

    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    #[error("API error: {0}")]
    Api(String),
}

impl LeakCheckError {
    pub fn from_status(status: u16, message: &str) -> Self {
        match status {
            400 if message.contains("Invalid type") => Self::InvalidType,
            400 if message.contains("Invalid email") => Self::InvalidEmail,
            400 if message.contains("Invalid query") => Self::InvalidQuery,
            400 if message.contains("Invalid domain") => Self::InvalidDomain,
            400 if message.contains("Too short") => Self::QueryTooShort,
            400 if message.contains("Invalid characters") => Self::InvalidCharacters,
            400 => Self::InvalidQuery,
            401 => Self::MissingApiKey,
            403 if message.contains("Limit reached") => Self::QuotaReached,
            403 => Self::PlanRequired,
            429 => Self::RateLimited,
            422 => Self::UnknownType,
            _ => Self::Api(format!("{status}: {message}")),
        }
    }
}