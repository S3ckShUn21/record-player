use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SpotifyAuthEnvVars {
    pub client_id: String,
    pub scope: String,
    pub redirect_uri: String,
}