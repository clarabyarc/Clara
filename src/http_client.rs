// Import error handling
use anyhow::Result;
// Import HTTP POST request function
use ureq::post;

// HTTP client structure for making requests
#[derive(Debug)]
pub struct HttpClient;

impl HttpClient {
    // Create new HTTP client instance
    pub fn new() -> Self {
        HttpClient
    }

    // Make POST request with JSON body
    pub fn post(&self, url: &str, body: serde_json::Value) -> Result<serde_json::Value> {
        let response = post(url).send_json(body)?;
        Ok(response.into_json()?)
    }

    // Make authenticated POST request with JSON body
    pub fn post_with_auth(&self, url: &str, access_token: &str, body: serde_json::Value) -> Result<String> {
        let response = post(url)
            .set("Authorization", &format!("Bearer {}", access_token))
            .send_json(body)?;
        Ok(response.into_string()?)
    }
}
