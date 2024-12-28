// Import standard library modules
use std::{env, process};

// Import Twitter client related dependencies
use agent_twitter_client::{models::Profile, scraper::Scraper, search::SearchMode};
// Import logging and error handling
use log::error;
// Import serialization/deserialization traits
use serde::{Deserialize, Serialize};
use serde_json::Value;
use anyhow::Result;

// Main Twitter client struct
pub struct Twitter {
    // Twitter account username
    pub username: String,
    // Twitter account password
    pub password: String,
    // Twitter account email
    pub email: String,
    // Twitter scraper instance
    pub scraper: Scraper,
}

// Structure representing extracted tweet data
#[derive(Debug, Serialize, Deserialize)]
pub struct ExtractedTweet {
    // User's display name
    pub name: Option<String>,
    // User's Twitter handle
    pub username: Option<String>,
    // Unique user identifier
    pub user_id: Option<String>,
    // Tweet content
    pub text: Option<String>,
    // Tweet timestamp
    pub timestamp: Option<i64>,
    // Permanent URL to the tweet
    pub permanent_url: Option<String>,
    // Unique tweet identifier
    pub id: Option<String>,
}

impl Twitter {
    // Initialize new Twitter client instance
    pub async fn new() -> Result<Self> {
        // Get Twitter credentials from environment variables
        let username = env::var("TWITTER_USERNAME").unwrap_or_else(|err| {
            error!("Missing TWITTER_USERNAME {}", err);
            process::exit(1);
        });

        let password = env::var("TWITTER_PASSWORD").unwrap_or_else(|err| {
            error!("Missing TWITTER_PASSWORD {}", err);
            process::exit(1);
        });

        let email = env::var("TWITTER_EMAIL").unwrap_or_else(|err| {
            error!("Missing TWITTER_EMAIL {}", err);
            process::exit(1);
        });

        // Initialize and login to Twitter
        let mut scraper = Scraper::new().await?;

        scraper
            .login(username.clone(), password.clone(), Some(email.clone()), None)
            .await?;

        Ok(Self {
            username,
            password,
            email,
            scraper,
        })
    }

    // Search for tweets matching query
    pub async fn search_tweets(
        &self,
        query: &str,
        max_tweets: i32,
        search_mode: Option<SearchMode>,
        cursor: Option<String>,
    ) -> Result<Vec<ExtractedTweet>> {
        // Perform tweet search
        let tweets = self
            .scraper
            .search_tweets(query, max_tweets, search_mode.unwrap_or(SearchMode::Latest), cursor)
            .await?;

        // Convert tweets to ExtractedTweet format
        let extracted_tweets: Vec<ExtractedTweet> = tweets
            .tweets
            .iter()
            .map(|tweet| ExtractedTweet {
                name: tweet.name.clone(),
                username: tweet.username.clone(),
                user_id: tweet.user_id.clone(),
                text: tweet.text.clone(),
                timestamp: tweet.timestamp,
                permanent_url: tweet.permanent_url.clone(),
                id: tweet.id.clone(),
            })
            .collect();

        Ok(extracted_tweets)
    }

    // Get user profile information
    pub async fn get_profile(&self, username: &str) -> Result<Profile> {
        let profile = self.scraper.get_profile(username).await?;
        Ok(profile)
    }

    // Get user's avatar URL from profile
    pub async fn get_avatar(&self, profile: Profile) -> Result<Option<String>> {
        Ok(profile.profile_image_url)
    }

    // Send a new tweet with optional media
    pub async fn send_tweet(
        &self,
        text: &str,
        reply_to: Option<&str>,
        media_data: Option<Vec<(Vec<u8>, String)>>,
    ) -> Result<Value> {
        let tweet_with_media = self.scraper.send_tweet(text, reply_to, media_data).await?;
        Ok(tweet_with_media)
    }
}
