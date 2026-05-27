use serde::{Deserialize, Serialize};

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

#[derive(Debug, Clone, Deserialize)]
pub struct QueryResponse {
    pub success: bool,
    pub found: i32,
    pub quota: i32,
    pub result: Vec<LeakResult>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PublicSource {
    pub name: String,
    pub date: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PublicQueryResponse {
    pub success: bool,
    pub found: i32,
    #[serde(default)]
    pub fields: Vec<String>,
    #[serde(default)]
    pub sources: Vec<PublicSource>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ApiErrorResponse {
    pub success: bool,
    pub error: String,
    #[serde(default)]
    pub found: i32,
    #[serde(default)]
    pub quota: i32,
}

pub const MAX_LIMIT: u32 = 1000;
pub const MAX_OFFSET: u32 = 2500;

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

    pub fn query_type(mut self, query_type: QueryType) -> Self {
        self.query_type = query_type;
        self
    }

    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit.min(MAX_LIMIT));
        self
    }

    pub fn offset(mut self, offset: u32) -> Self {
        self.offset = Some(offset.min(MAX_OFFSET));
        self
    }
}