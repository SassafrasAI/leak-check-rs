//! # leak-check-rs
//!
//! Rust client for the [LeakCheck API](https://leakcheck.io) v2.
//!
//! Supports both the authenticated Pro API and the free Public API.
//!
//! ## Quick Start
//!
//! ```ignore
//! use leak_check_rs::{LeakCheckClient, LeakCheckPublicClient, QueryOptions, QueryType};
//!
//! // Pro API (authenticated)
//! let client = LeakCheckClient::new("your_api_key");
//! let result = client.query("user@example.com").await?;
//!
//! // With options
//! let result = client.query_with_options("gmail.com",
//!     QueryOptions::new().query_type(QueryType::Domain).limit(100)
//! ).await?;
//!
//! // Fetch all results across pages
//! let all = client.query_all("gmail.com", QueryType::Domain).await?;
//!
//! // Public API (no key needed)
//! let public = LeakCheckPublicClient::new();
//! let result = public.query("user@example.com").await?;
//! ```

pub mod client;
pub mod error;
pub mod types;

pub use client::{LeakCheckClient, LeakCheckPublicClient};
pub use error::LeakCheckError;
pub use types::{
    ApiErrorResponse, LeakResult, PublicQueryResponse, PublicSource, QueryOptions, QueryResponse,
    QueryType, Source, MAX_LIMIT, MAX_OFFSET,
};