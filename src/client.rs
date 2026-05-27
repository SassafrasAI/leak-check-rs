use crate::error::LeakCheckError;
use crate::types::{PublicQueryResponse, QueryOptions, QueryResponse, QueryType, MAX_LIMIT};

const API_BASE_URL: &str = "https://leakcheck.io/api/v2/query";
const PUBLIC_API_URL: &str = "https://leakcheck.io/api/public";

/// Client for the LeakCheck Pro API v2 (authenticated).
///
/// Requires an API key obtainable from <https://leakcheck.io> account settings.
/// The key is sent via the `X-API-Key` header.
///
/// Rate limited to 3 requests/second by default (upgradeable).
pub struct LeakCheckClient {
    client: reqwest::Client,
    api_key: String,
}

impl LeakCheckClient {
    /// Create a new client with the given API key.
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key: api_key.into(),
        }
    }

    /// Create a client with a custom `reqwest::Client` (e.g. for proxy, TLS config).
    pub fn with_http_client(api_key: impl Into<String>, client: reqwest::Client) -> Self {
        Self {
            client,
            api_key: api_key.into(),
        }
    }

    /// Query with auto-detected type and no pagination.
    ///
    /// The API will infer the query type (email, username, phone, or hash).
    /// Returns up to 100 results by default.
    pub async fn query(&self, query: &str) -> Result<QueryResponse, LeakCheckError> {
        self.query_with_options(query, QueryOptions::default()).await
    }

    /// Query with explicit type, limit, and offset.
    ///
    /// Use [`QueryOptions::new()`] to build options with the builder pattern.
    /// Limit is clamped to [`MAX_LIMIT`] (1000), offset to [`crate::MAX_OFFSET`] (2500).
    pub async fn query_with_options(
        &self,
        query: &str,
        options: QueryOptions,
    ) -> Result<QueryResponse, LeakCheckError> {
        let url = format!("{API_BASE_URL}/{query}");

        let mut request = self
            .client
            .get(&url)
            .header("X-API-Key", &self.api_key)
            .header("Accept", "application/json");

        if options.query_type != QueryType::Auto {
            request = request.query(&[("type", options.query_type.to_string())]);
        }
        if let Some(limit) = options.limit {
            request = request.query(&[("limit", limit.to_string())]);
        }
        if let Some(offset) = options.offset {
            request = request.query(&[("offset", offset.to_string())]);
        }

        let response = request.send().await?;
        let status = response.status().as_u16();
        let body = response.text().await?;

        if status >= 400 {
            if let Ok(api_error) = serde_json::from_str::<crate::types::ApiErrorResponse>(&body) {
                return Err(LeakCheckError::from_status(status, &api_error.error));
            }
            return Err(LeakCheckError::Api(format!("{status}: {body}")));
        }

        let query_response: QueryResponse = serde_json::from_str(&body)
            .map_err(|e| LeakCheckError::Api(format!("Failed to parse response: {e}")))?;

        Ok(query_response)
    }

    /// Fetch all results for a query, automatically paginating until complete.
    ///
    /// Uses the maximum page size and increments offset until all results are retrieved.
    pub async fn query_all(&self, query: &str, query_type: QueryType) -> Result<Vec<crate::types::LeakResult>, LeakCheckError> {
        let mut all_results = Vec::new();
        let mut offset = 0u32;

        loop {
            let options = QueryOptions::new()
                .query_type(query_type)
                .limit(MAX_LIMIT)
                .offset(offset);

            let response = self.query_with_options(query, options).await?;
            let count = response.result.len();
            all_results.extend(response.result);

            if count < MAX_LIMIT as usize || (offset as usize) + count >= response.found as usize {
                break;
            }
            offset += MAX_LIMIT;
        }

        Ok(all_results)
    }
}

/// Client for the LeakCheck Public API (unauthenticated, no key needed).
///
/// Limited to email, email hash, and username lookups.
/// Does not return passwords or full breach data.
/// Rate limited to 1 request/second.
pub struct LeakCheckPublicClient {
    client: reqwest::Client,
}

impl LeakCheckPublicClient {
    /// Create a new public API client.
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    /// Create a public client with a custom `reqwest::Client`.
    pub fn with_http_client(client: reqwest::Client) -> Self {
        Self { client }
    }

    /// Query the public API for an email, username, or email hash.
    ///
    /// The type is auto-detected. Returns source names and breach dates
    /// but not passwords or full field data.
    pub async fn query(&self, query: &str) -> Result<PublicQueryResponse, LeakCheckError> {
        let response = self
            .client
            .get(PUBLIC_API_URL)
            .header("Accept", "application/json")
            .query(&[("check", query)])
            .send()
            .await?;

        let status = response.status().as_u16();
        let body = response.text().await?;

        if status >= 400 {
            if let Ok(api_error) = serde_json::from_str::<crate::types::ApiErrorResponse>(&body) {
                return Err(LeakCheckError::from_status(status, &api_error.error));
            }
            return Err(LeakCheckError::Api(format!("{status}: {body}")));
        }

        let query_response: PublicQueryResponse = serde_json::from_str(&body)
            .map_err(|e| LeakCheckError::Api(format!("Failed to parse response: {e}")))?;

        Ok(query_response)
    }
}

impl Default for LeakCheckPublicClient {
    fn default() -> Self {
        Self::new()
    }
}