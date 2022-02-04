use dotenv;
use std::path::Path;

//
// Public Functions
//

// Check for credentials (from .env)
pub fn check_for_access_token() {
    let cache_dir = dotenv::var("CACHE_DIR").unwrap();
    let tokens_path = Path::new(&cache_dir).join("tokens");

    println!("{}", tokens_path.exists());
}

// Generate the login request

// Get tokens from code

// Save Tokens to the file

//
// Spotify Client
//
pub struct SpotifyClient {}
