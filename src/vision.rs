// Import required dependencies
use anyhow::Result;
// Import JWT related modules
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
// Import serialization traits
use serde::{Deserialize, Serialize};
use serde_json::Value;
// Import time related modules
use std::time::{SystemTime, UNIX_EPOCH};
// Import JSON macro
use ureq::json;

// Import local modules
use crate::{http_client::HttpClient, image::Image};

// Constants for API endpoints and scopes
const VISION_API_URL: &'static str = "https://vision.googleapis.com/v1/images:annotate";
const CLOULD_PLATFORM_URL: &'static str = "https://www.googleapis.com/auth/cloud-platform";
const CLOULD_TOKEN_URL: &'static str = "https://oauth2.googleapis.com/token";

// JWT claims structure for Google authentication
#[derive(Debug, Serialize)]
struct Claims {
    // Service account email
    iss: String,
    // Space-separated list of scopes
    scope: String,
    // Authentication endpoint
    aud: String,
    // Expiration time
    exp: usize,
    // Issued at time
    iat: usize,
}

// Response structure for Vision API
#[derive(Debug, Serialize, Deserialize)]
pub struct Response {
    pub responses: Vec<LabelAnnotationsResponse>,
}

// Structure for label annotations response
#[derive(Debug, Serialize, Deserialize)]
pub struct LabelAnnotationsResponse {
    #[serde(rename = "labelAnnotations")]
    pub label_annotations: Vec<LabelAnnotation>,
}

// Structure for individual label annotation
#[derive(Debug, Serialize, Deserialize)]
pub struct LabelAnnotation {
    // Machine-generated ID
    pub mid: String,
    // Human-readable description
    pub description: String,
    // Confidence score
    pub score: f64,
    // Relevance to the overall image
    pub topicality: f64,
}

// Structure for Vision API request
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GoogleVisionRequest {
    // Image to analyze
    pub image: Image,
    // Maximum number of results to return
    pub max_results: u8,
}

// Main Google Vision API client
#[derive(Debug)]
pub struct GoogleVision {
    // Service account email
    client_email: String,
    // Private key for authentication
    private_key: String,
    // HTTP client instance
    http_client: HttpClient,
}

impl GoogleVision {
    // Initialize new Vision API client
    pub fn new() -> Result<Self> {
        // Load service account credentials
        let service_account_key: Value = serde_json::from_str(&std::fs::read_to_string("service_account.json")?)?;
        let client_email = service_account_key["client_email"].as_str().unwrap();
        let private_key = service_account_key["private_key"].as_str().unwrap();

        Ok(Self {
            client_email: client_email.into(),
            private_key: private_key.into(),
            http_client: HttpClient::new(),
        })
    }

    // Generate image descriptions using Vision API
    pub fn create_desc(&self, request: GoogleVisionRequest) -> Result<Vec<String>> {
        // Get current timestamp
        let now = SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as usize;
        
        // Create JWT claims
        let claims = Claims {
            iss: self.client_email.clone(),
            scope: CLOULD_PLATFORM_URL.to_string(),
            aud: CLOULD_TOKEN_URL.to_string(),
            exp: now + 3600, // Token expires in 1 hour
            iat: now,
        };

        // Generate JWT token
        let header = Header::new(Algorithm::RS256);
        let encoding_key = EncodingKey::from_rsa_pem(self.private_key.clone().as_bytes())?;
        let jwt = encode(&header, &claims, &encoding_key)?;

        // Get access token
        let response: Value = self.http_client.post(
            CLOULD_TOKEN_URL,
            json!({
                "grant_type": "urn:ietf:params:oauth:grant-type:jwt-bearer",
                "assertion": jwt
            }),
        )?;

        let access_token = response["access_token"].to_string();

        // Make Vision API request
        let response = self.http_client.post_with_auth(
            VISION_API_URL,
            &access_token,
            json!({
              "requests": [
                {
                  "image": {
                    "content": request.image.base64
                  },
                  "features": [
                    {
                      "type": "LABEL_DETECTION",
                      "maxResults": request.max_results
                    }
                  ]
                }
              ]
            }),
        )?;

        // Parse and sort response
        let response: Response = serde_json::from_str(&response)?;
        let mut descriptions: Vec<(&str, f64)> = response.responses[0]
            .label_annotations
            .iter()
            .map(|annotation| (&annotation.description[..], annotation.score))
            .collect();
        descriptions.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
        let sorted_descriptions: Vec<String> = descriptions.into_iter().map(|(desc, _)| desc.to_string()).collect();

        Ok(sorted_descriptions)
    }
}
