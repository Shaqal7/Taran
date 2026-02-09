use crate::error::{ProtocolError, Result};
use reqwest::{Client, Method};
use serde_json::Value;
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// HTTP client wrapper around reqwest
#[derive(Debug, Clone)]
pub struct HttpClient {
    client: Client,
}

/// HTTP request configuration
#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub method: String,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub body: Option<String>,
    pub timeout: Option<Duration>,
}

/// HTTP response with metadata
#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub duration: Duration,
    pub bytes_sent: u64,
    pub bytes_received: u64,
}

impl HttpClient {
    /// Create a new HTTP client
    pub fn new() -> Result<Self> {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| ProtocolError::ConnectionError(e.to_string()))?;

        Ok(Self { client })
    }

    /// Execute an HTTP request
    pub async fn execute(&self, request: HttpRequest) -> Result<HttpResponse> {
        let start = Instant::now();

        let method = parse_method(&request.method)?;
        let mut req_builder = self.client.request(method, &request.url);

        // Add headers
        for (key, value) in &request.headers {
            req_builder = req_builder.header(key, value);
        }

        // Add body if present
        if let Some(body) = &request.body {
            req_builder = req_builder.body(body.clone());
        }

        // Set timeout if specified
        if let Some(timeout) = request.timeout {
            req_builder = req_builder.timeout(timeout);
        }

        // Send request
        let response = req_builder
            .send()
            .await
            .map_err(|e| ProtocolError::HttpRequestFailed(e.to_string()))?;

        let status = response.status().as_u16();

        // Extract headers
        let mut headers = HashMap::new();
        for (key, value) in response.headers() {
            if let Ok(value_str) = value.to_str() {
                headers.insert(key.to_string(), value_str.to_string());
            }
        }

        // Read body
        let body_bytes =
            response.bytes().await.map_err(|e| ProtocolError::HttpRequestFailed(e.to_string()))?;

        let body = String::from_utf8_lossy(&body_bytes).to_string();
        let bytes_received = body_bytes.len() as u64;

        // Estimate bytes sent (rough approximation)
        let bytes_sent = estimate_request_size(&request);

        let duration = start.elapsed();

        Ok(HttpResponse { status, headers, body, duration, bytes_sent, bytes_received })
    }

    /// Convenience method for GET request
    pub async fn get(&self, url: &str) -> Result<HttpResponse> {
        self.execute(HttpRequest {
            method: "GET".to_string(),
            url: url.to_string(),
            headers: HashMap::new(),
            body: None,
            timeout: None,
        })
        .await
    }

    /// Convenience method for POST request
    pub async fn post(&self, url: &str, body: String) -> Result<HttpResponse> {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());

        self.execute(HttpRequest {
            method: "POST".to_string(),
            url: url.to_string(),
            headers,
            body: Some(body),
            timeout: None,
        })
        .await
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new().expect("Failed to create default HTTP client")
    }
}

impl HttpResponse {
    /// Parse response body as JSON
    pub fn json(&self) -> Result<Value> {
        serde_json::from_str(&self.body)
            .map_err(|e| ProtocolError::InvalidResponse(format!("JSON parse error: {}", e)))
    }

    /// Check if response is successful (2xx status code)
    pub fn is_success(&self) -> bool {
        self.status >= 200 && self.status < 300
    }

    /// Get a header value
    pub fn header(&self, key: &str) -> Option<&String> {
        self.headers.get(key)
    }
}

fn parse_method(method: &str) -> Result<Method> {
    match method.to_uppercase().as_str() {
        "GET" => Ok(Method::GET),
        "POST" => Ok(Method::POST),
        "PUT" => Ok(Method::PUT),
        "DELETE" => Ok(Method::DELETE),
        "PATCH" => Ok(Method::PATCH),
        "HEAD" => Ok(Method::HEAD),
        "OPTIONS" => Ok(Method::OPTIONS),
        _ => Err(ProtocolError::InvalidResponse(format!("Unsupported HTTP method: {}", method))),
    }
}

fn estimate_request_size(request: &HttpRequest) -> u64 {
    let mut size = 0u64;

    // Method + URL + HTTP version
    size += request.method.len() as u64;
    size += request.url.len() as u64;
    size += 10; // " HTTP/1.1\r\n"

    // Headers
    for (key, value) in &request.headers {
        size += key.len() as u64 + value.len() as u64 + 4; // ": " + "\r\n"
    }
    size += 2; // Final "\r\n"

    // Body
    if let Some(body) = &request.body {
        size += body.len() as u64;
    }

    size
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_method() {
        assert!(parse_method("GET").is_ok());
        assert!(parse_method("POST").is_ok());
        assert!(parse_method("get").is_ok());
        assert!(parse_method("INVALID").is_err());
    }

    #[test]
    fn test_estimate_request_size() {
        let req = HttpRequest {
            method: "GET".to_string(),
            url: "https://example.com".to_string(),
            headers: HashMap::new(),
            body: None,
            timeout: None,
        };
        let size = estimate_request_size(&req);
        assert!(size > 0);
    }
}
