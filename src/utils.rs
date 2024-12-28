use std::{env, fs, path::PathBuf};

use directories_next::ProjectDirs;
use uuid::Uuid;

// Generate custom image path in current directory
pub fn custom_image_path() -> PathBuf {
    // Get current working directory
    let current_dir = env::current_dir().unwrap();
    let image_dir = current_dir.join("images");

    // Create images directory if it doesn't exist
    if !image_dir.exists() {
        fs::create_dir_all(&image_dir).unwrap();
    }

    // Generate unique filename using UUID
    let unique_file_name = format!("image-{}.png", Uuid::new_v4());
    let unique_path = image_dir.join(unique_file_name);

    unique_path
}

// Generate image path in application data directory
pub fn generate_image_path() -> PathBuf {
    // Get application-specific directory
    let main_dirs = ProjectDirs::from("", "", "clara").unwrap();
    let image_dir = main_dirs.data_local_dir().join("images");

    // Create images directory if it doesn't exist
    fs::create_dir_all(&image_dir).ok().unwrap();

    // Generate unique filename using UUID
    let unique_file_name = format!("image-{}.png", Uuid::new_v4());
    let unique_path = image_dir.join(unique_file_name);

    unique_path
}
