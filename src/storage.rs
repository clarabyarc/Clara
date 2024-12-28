// Import serialization/deserialization traits
use serde::{Deserialize, Serialize};
// Import JSON handling utilities
use serde_json::{self, Error};
// Import HashSet for storing unique items
use std::collections::HashSet;
// Import file system operations
use std::fs::{File, OpenOptions};
// Import I/O utilities for buffered reading/writing
use std::io::{self, BufReader, BufWriter};

// Structure for persistent storage of processed items
#[derive(Serialize, Deserialize)]
pub struct Storage {
    // Path to storage file
    file_path: String,
    // Set of stored items (tweet IDs)
    items: HashSet<String>,
}

impl Storage {
    // Load storage from file, create new if file doesn't exist
    pub fn load_from_file(file_path: &str) -> Result<Self, Error> {
        // Open existing file or create new one
        let file = File::open(file_path).unwrap_or_else(|_| File::create(file_path).unwrap());
        // Create buffered reader for efficient reading
        let reader = BufReader::new(file);
        // Try to deserialize existing data or create empty storage
        serde_json::from_reader(reader).or_else(|_| {
            Ok(Storage {
                file_path: file_path.to_string(),
                items: HashSet::new(),
            })
        })
    }

    // Save current storage state to file
    pub fn save_to_file(&self) -> io::Result<()> {
        // Open file with write permissions, create if doesn't exist
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&self.file_path)?;
        // Create buffered writer for efficient writing
        let writer = BufWriter::new(file);
        // Serialize and write storage to file
        serde_json::to_writer(writer, &self).map_err(|e| io::Error::new(io::ErrorKind::Other, e))
    }

    // Insert new item into storage
    pub fn insert(&mut self, tweet: String) -> bool {
        // Returns true if item was newly inserted
        self.items.insert(tweet)
    }

    // Check if item exists in storage
    pub fn contains(&mut self, tweet: String) -> bool {
        // Returns true if item exists
        self.items.contains(&tweet)
    }

    // Remove item from storage
    pub fn remove(&mut self, tweet: String) -> bool {
        // Returns true if item was present and removed
        self.items.remove(&tweet)
    }
}
