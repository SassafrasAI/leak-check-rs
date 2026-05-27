use serde::{Deserialize, Serialize};

/// Type of search query for the Pro API.
///
/// Use [`QueryType::Auto`] to let the API infer the type from the query string,
/// or specify explicitly for domain, keyword, phone, etc.
///
/// `Origin` and `Password` require an Enterprise plan.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum QueryType {
    Auto,
    Email,
    Domain,
    Keyword,
    Username,
    Phone,
    Hash,
    Phash,
    Origin,
    Password,
}

impl Default for QueryType {
    fn default() -> Self {
        Self::Auto
    }
}

impl std::fmt::Display for QueryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = serde_json::to_value(self)
            .map_err(|_| std::fmt::Error)?;
        s.as_str().unwrap_or("auto").fmt(f)
    }
}

/// Breach source metadata from the Pro API.
#[derive(Debug, Clone, Deserialize)]
pub struct Source {
    pub name: String,
    #[serde(rename = "breach_date", default)]
    pub breach_date: Option<String>,
    #[serde(default)]
    pub unverified: i32,
    #[serde(default)]
    pub passwordless: i32,
    #[serde(default)]
    pub compilation: i32,
}

/// A single leaked credential record from the Pro API.
#[derive(Debug, Clone, Deserialize)]
pub struct LeakResult {
    pub email: Option<String>,
    pub source: Option<Source>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub username: Option<String>,
    #[serde(default)]
    pub fields: Vec<String>,
}

/// Response from the Pro API v2 query endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct QueryResponse {
    pub success: bool,
    /// Total number of matching records.
    pub found: i32,
    /// Remaining quota on the account.
    pub quota: i32,
    pub result: Vec<LeakResult>,
}

/// Breach source from the Public API.
#[derive(Debug, Clone, Deserialize)]
pub struct PublicSource {
    pub name: String,
    pub date: Option<String>,
}

/// Response from the Public API query endpoint.
#[derive(Debug, Clone, Deserialize)]
pub struct PublicQueryResponse {
    pub success: bool,
    pub found: i32,
    #[serde(default)]
    pub fields: Vec<String>,
    #[serde(default)]
    pub sources: Vec<PublicSource>,
}

/// Error response body from the API.
#[derive(Debug, Clone, Deserialize)]
pub struct ApiErrorResponse {
    pub success: bool,
    pub error: String,
    #[serde(default)]
    pub found: i32,
    #[serde(default)]
    pub quota: i32,
}

/// Maximum page size allowed by the API (1000).
pub const MAX_LIMIT: u32 = 1000;

/// Maximum offset allowed by the API (2500).
pub const MAX_OFFSET: u32 = 2500;

/// Options for a Pro API query.
///
/// Use the builder pattern: `QueryOptions::new().query_type(QueryType::Email).limit(100)`
///
/// Limit is clamped to [`MAX_LIMIT`], offset to [`MAX_OFFSET`].
#[derive(Debug, Clone)]
pub struct QueryOptions {
    pub query_type: QueryType,
    pub limit: Option<u32>,
    pub offset: Option<u32>,
}

impl Default for QueryOptions {
    fn default() -> Self {
        Self {
            query_type: QueryType::Auto,
            limit: None,
            offset: None,
        }
    }
}

impl QueryOptions {
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the query type (defaults to [`QueryType::Auto`]).
    pub fn query_type(mut self, query_type: QueryType) -> Self {
        self.query_type = query_type;
        self
    }

    /// Set the page size, clamped to [`MAX_LIMIT`].
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit.min(MAX_LIMIT));
        self
    }

    /// Set the offset, clamped to [`MAX_OFFSET`].
    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset.min(MAX_OFFSET));
        self
    }
}