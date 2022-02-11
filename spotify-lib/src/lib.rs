use serde_json;
use ureq;

use std::time;

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::PathBuf;
use std::time::UNIX_EPOCH;

//
// RFID Card
//

pub enum RecordType {
    Track,
    Episode,
    Playlist,
    Album,
    Artist,
}

pub struct Record {
    rec_type: RecordType, // This is to help with the different cases
    uri: String,          // This will be the uri without "spotify:"" prepended
    params: String,       // This will be JSON
}

// How will I use this???

// Creating a new card
// Generate a Record from the Spotify URI
// Export that Record to the RFID tag

// Playing the track
// Read the RFID card
// Convert the bytes to a Record
// check the rec_type
// use the requisite spotify.playTrack(Record);

//
// Spotify Api Object
//

/// Api Struct that will be used to talk to the spotify api endpoints (actually mess with spotify)
#[derive(Serialize, Deserialize)]
pub struct SpotifyApi {
    #[serde(skip)]
    file_path: PathBuf,
    api_client_id: String,
    access_token: String,
    refresh_token: String,
    expiration_date: time::Duration, // All durations will be in relation to UNIX_EPOCH
}

/// Wrapper Struct to help capture the https response from getting access tokens
#[derive(Serialize, Deserialize)]
struct AccessTokenResponse {
    access_token: String,
    token_type: String,
    scope: String,
    expires_in: u64,
    #[serde(default)]
    refresh_token: String, // This may not be here if we're requesting a refreshed access token from the token endpoint
}

// Methods
// playTrack
// playPlaylist
// playAlbum
// playPodcast
// either have an episode #
// or "first"
// or none and have it queue a bunch of random ones that the user can skip through until they find one they want
// playArtist

// check token expired
// update access token
// cache tokens

impl SpotifyApi {
    //
    // Constructor
    //

    /// Only call this function if you know the cache file exists
    pub fn new(cache_file: &PathBuf) -> Result<SpotifyApi, Box<dyn Error>> {
        let file = std::fs::File::open(cache_file)?;
        let mut s_api: SpotifyApi = serde_json::from_reader(file)?;
        s_api.file_path = cache_file.clone();
        Ok(s_api)
    }

    //
    // Public Functions
    //

    pub fn pause_playback(&mut self) -> Result<ureq::Response, Box<dyn Error>> {
        self.check_token_expiration()?;

        let res = ureq::put("https://api.spotify.com/v1/me/player/pause")
            .set(
                "Authorization",
                ["Bearer ", self.access_token.as_str()].join("").as_str(),
            )
            .set("Content-Type", "application/json")
            .set("Content-Length", "0")
            .call()?;
        Ok(res)
    }

    pub fn play_track(&mut self) -> Result<ureq::Response, Box<dyn Error>> {
        self.check_token_expiration()?;

        let res = ureq::put("https://api.spotify.com/v1/me/player/play")
            .set(
                "Authorization",
                ["Bearer ", self.access_token.as_str()].join("").as_str(),
            ) // TODO : Is this the best way to concat strs?
            .set("Content-Type", "application/json")
            .send_json(ureq::json!({
                "uris" : ["spotify:track:6o46wIUnoKS8UABL36UuLL"],
            }))?;
        Ok(res)
    }

    //
    // Private Helper Functions
    //

    fn check_token_expiration(&mut self) -> Result<(), Box<dyn Error>> {
        // Get 'now' in terms of a duration
        let func_now = time::SystemTime::now().duration_since(UNIX_EPOCH)?;

        // Check to see if the token will expire within 1 minute
        // This gives time to actually make the request while still being sure the token is valid
        if func_now < (self.expiration_date + time::Duration::from_secs(60)) {
            return Ok(());
        }

        // Otherwise the token will be invalid, so refresh it
        println!("Refreshing Token!!!");

        // Run the request to get another access_token
        let access_token_res: AccessTokenResponse =
            ureq::post("https://accounts.spotify.com/api/token")
                .set("Authorization", "Basic NTgxYTE3Mjc0YWRlNGFmYWI0MmU0ZDJjZThlOGI5NzE6YmY4ZDhhNTI3M2U3NDk4NGE0OGU2OGVkYWJjOGRjYWQ=") // TODO : Add client_secret to struct so I can calculate this on the fly
                .set("Content-Type", "application/x-www-form-urlencoded")
                .send_form(&[
                    ("grant_type", "refresh_token"),
                    ("refresh_token", self.refresh_token.as_str()),
                ])?
                .into_json()?;
        // Use the JSON to modify the spotify api
        self.access_token = access_token_res.access_token;
        self.expiration_date = func_now + time::Duration::from_secs(access_token_res.expires_in);

        println!("Attempting to serialize");

        println!("{:?}", &self.file_path);

        // Cache the SpotifyApi struct
        let cache_file = std::fs::File::create(&self.file_path)?; // Overwrite the existing file
        serde_json::to_writer(cache_file, self)?;

        // Return
        Ok(())
    }
}

// spotify:track:6o46wIUnoKS8UABL36UuLL (Get You High - HARBOUR)
