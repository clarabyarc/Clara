// Import local modules for HTTP client and image handling
use crate::{
    http_client::HttpClient,
    image::{Image, ImageGenerator, ImageRequest},
};
// Import error handling and logging
use anyhow::Result;
use log::error;
// Import serialization traits
use serde::{Deserialize, Serialize};
// Import JSON macro and environment handling
use ureq::json;
use std::{env, process};

// OpenAI API endpoint for image generation
const OPENAI_IMAGE_GEN_URL: &'static str = "https://api.openai.com/v1/images/generations";

// Structure to hold OpenAI API response for image generation
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Images {
    // Timestamp of image creation
    pub created: u64,
    // Vector of generated images
    pub data: Option<Vec<ImageData>>,
}

// Structure to hold individual image data from OpenAI
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ImageData {
    // Base64 encoded image data
    pub b64_json: String,
}

// Main image generation client
pub struct ImageGen {
    // OpenAI API key
    key: String,
    // HTTP client instance
    http_client: HttpClient,
}

impl ImageGen {
    // Initialize new image generation client
    pub fn new() -> Result<Self> {
        // Get OpenAI API key from environment variables
        let key = env::var("OPENAI_API_KEY").unwrap_or_else(|err| {
            error!("Missing OPENAI_API_KEY {}", err);
            process::exit(1);
        });

        Ok(Self {
            key,
            http_client: HttpClient::new(),
        })
    }
}

// Implementation of ImageGenerator trait for DALL-E
impl ImageGenerator for ImageGen {
    // Create image using DALL-E model
    fn create_image(&self, request: ImageRequest) -> Result<Image> {
        // Make request to OpenAI API
        let response = self.http_client.post_with_auth(
            OPENAI_IMAGE_GEN_URL,
            &self.key,
            json!({
              "prompt": request.description,
              "n": 1,                           // Generate one image
              "response_format": "b64_json",    // Request base64 encoded response
              "model": "dall-e-3",             // Use DALL-E 3 model
              "quality": "hd",                 // Request high quality image
              "size": format!("{}x{}", request.width, request.height), // Set image dimensions
            }),
        )?;

        // Parse response and extract image data
        let images: Images = serde_json::from_str(&response)?;
        let image = &images.data.unwrap()[0];
        let base64 = image.b64_json.clone();

        // Create and return Image instance
        Ok(Image::from_base64(base64))
    }
}
