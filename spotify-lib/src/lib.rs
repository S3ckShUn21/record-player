use serde_json;
use ureq;

use std::time;

use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::PathBuf;
use std::time::UNIX_EPOCH;

use base64::encode;

use dotenv;

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
    basic_auth_string: String,
    access_token: String,
    refresh_token: String,
    expiration_date: time::Duration, // All durations will be in relation to UNIX_EPOCH
}

/// Wrapper Struct to help capture the https response from getting access tokens
#[derive(Serialize, Deserialize)]
pub struct AccessTokenResponse {
    access_token: String,
    token_type: String,
    scope: String,
    expires_in: u64,
    #[serde(default)]
    refresh_token: String, // This may not be here if we're requesting a refreshed access token from the token endpoint
}

//
// Module Functions
//

pub fn get_access_token_response(code: &str) -> Result<AccessTokenResponse, Box<dyn Error>> {
    let auth_string = encode_basic_auth_string(
        dotenv::var("CLIENT_ID").unwrap().as_str(),
        dotenv::var("CLIENT_SECRET").unwrap().as_str(),
    );

    let res: AccessTokenResponse = ureq::post("https://accounts.spotify.com/api/token")
        .set("Authorization", auth_string.as_str())
        .set("Content-Type", "application/x-www-form-urlencoded")
        .send_form(&[
            ("grant_type", "authorization_code"),
            ("code", code),
            (
                "redirect_uri",
                dotenv::var("REDIRECT_URI").unwrap().as_str(),
            ),
        ])?
        .into_json()?;

    Ok(res)
}

fn encode_basic_auth_string(client_id: &str, client_secret: &str) -> String {
    let mut concat = client_id.to_owned();
    concat.push(':');
    concat.push_str(client_secret);
    let encoded = encode(concat);
    let mut final_str = "Basic ".to_owned();
    final_str.push_str(encoded.as_str());
    final_str
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

    pub fn from_token_data(cache_file: &PathBuf, token_data: AccessTokenResponse) -> SpotifyApi {
        let func_now = time::SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        let expiration_date = func_now + time::Duration::from_secs(token_data.expires_in);

        let auth_string = encode_basic_auth_string(
            dotenv::var("CLIENT_ID").unwrap().as_str(),
            dotenv::var("CLIENT_SECRET").unwrap().as_str(),
        );

        SpotifyApi {
            file_path: cache_file.clone(),
            basic_auth_string: auth_string,
            access_token: token_data.access_token,
            refresh_token: token_data.refresh_token,
            expiration_date,
        }
    }

    /// Only call this function if you know the cache file exists
    pub fn read(cache_file: &PathBuf) -> Result<SpotifyApi, Box<dyn Error>> {
        let file = std::fs::File::open(cache_file)?;
        let mut s_api: SpotifyApi = serde_json::from_reader(file)?;
        s_api.file_path = cache_file.clone();
        Ok(s_api)
    }

    //
    // Helper Functions
    //

    pub fn refresh(&mut self, token_data: AccessTokenResponse) {
        let func_now = time::SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        self.access_token = token_data.access_token;
        self.expiration_date = func_now + time::Duration::from_secs(token_data.expires_in);
    }

    pub fn cache(&self) -> Result<(), Box<dyn Error>> {
        let cache_file = std::fs::File::create(&self.file_path)?; // Overwrite the existing file
        serde_json::to_writer(cache_file, self)?;
        Ok(())
    }

    //
    // Endpoint Functions
    //

    pub fn pause_playback(&mut self) -> Result<ureq::Response, Box<dyn Error>> {
        self.check_token_expiration()?;

        let res = ureq::put("https://api.spotify.com/v1/me/player/pause")
            .set("Authorization", self.basic_auth_string.as_str())
            .set("Content-Type", "application/json")
            .set("Content-Length", "0")
            .call()?;
        Ok(res)
    }

    pub fn play_track(&mut self) -> Result<ureq::Response, Box<dyn Error>> {
        self.check_token_expiration()?;

        let res = ureq::put("https://api.spotify.com/v1/me/player/play")
            .set("Authorization", self.basic_auth_string.as_str())
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

        // Run the request to get another access_token
        let access_token_res: AccessTokenResponse =
            ureq::post("https://accounts.spotify.com/api/token")
                .set("Authorization", self.basic_auth_string.as_str())
                .set("Content-Type", "application/x-www-form-urlencoded")
                .send_form(&[
                    ("grant_type", "refresh_token"),
                    ("refresh_token", self.refresh_token.as_str()),
                ])?
                .into_json()?;

        // Update and re-cache
        self.refresh(access_token_res);
        self.cache().ok();

        // Return
        Ok(())
    }
}

// spotify:track:6o46wIUnoKS8UABL36UuLL (Get You High - HARBOUR)
