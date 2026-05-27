use crate::error::LeakCheckError;
use crate::types::{PublicQueryResponse, QueryOptions, QueryResponse, QueryType, MAX_LIMIT};

const API_BASE_URL: &str = "https://leakcheck.io/api/v2/query";
const PUBLIC_API_URL: &str = "https://leakcheck.io/api/public";

pub struct LeakCheckClient {
    client: reqwest::Client,
    api_key: String,
}

impl LeakCheckClient {
    pub fn new(api_key: impl Into<String>) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_key: api_key.into(),
        }
    }

    pub fn with_http_client(api_key: impl Into<String>, client: reqwest::Client) -> Self {
        Self {
            client,
            api_key: api_key.into(),
        }
    }

    pub async fn query(&self, query: &str) -> Result<QueryResponse, LeakCheckError> {
        self.query_with_options(query, QueryOptions::default()).await
    }

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

pub struct LeakCheckPublicClient {
    client: reqwest::Client,
}

impl LeakCheckPublicClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    pub fn with_http_client(client: reqwest::Client) -> Self {
        Self { client }
    }

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