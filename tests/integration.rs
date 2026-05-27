use leak_check_rs::{LeakCheckClient, LeakCheckPublicClient, QueryOptions, QueryType};

#[tokio::test]
async fn test_query_email() {
    let api_key = std::env::var("LEAKCHECK_API_KEY").expect("LEAKCHECK_API_KEY not set");
    let client = LeakCheckClient::new(&api_key);
    let result = client.query("test@example.com").await;
    assert!(result.is_ok(), "query failed: {:?}", result.err());
    let response = result.unwrap();
    assert!(response.success);
}

#[tokio::test]
async fn test_query_with_type() {
    let api_key = std::env::var("LEAKCHECK_API_KEY").expect("LEAKCHECK_API_KEY not set");
    let client = LeakCheckClient::new(&api_key);
    let options = QueryOptions {
        query_type: QueryType::Email,
        ..Default::default()
    };
    let result = client.query_with_options("test@example.com", options).await;
    assert!(result.is_ok(), "query failed: {:?}", result.err());
    let response = result.unwrap();
    assert!(response.success);
}

#[tokio::test]
async fn test_invalid_api_key() {
    let client = LeakCheckClient::new("invalid_key_that_is_at_least_40_characters_long_xx");
    let result = client.query("test@example.com").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_public_query() {
    let client = LeakCheckPublicClient::new();
    let result = client.query("test@example.com").await;
    assert!(result.is_ok(), "public query failed: {:?}", result.err());
    let response = result.unwrap();
    assert!(response.success);
}