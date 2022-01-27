use reqwest;

#[allow(dead_code)]
pub enum SkipDirection {
    Next,
    Prev
}

#[allow(dead_code)]
pub struct Spotify {
    client: reqwest::Client
}

#[allow(dead_code)]
impl Spotify {

    pub fn new() -> Spotify {
        Spotify {
            client: reqwest::Client::new()
        }
    }

    pub fn test(&self) {
        println!("Hello from Spotify Struct!");
    }
    
    /// Switch the playback to this URI immediately on the currently active device
    pub fn play_uri(&self, uri: &str) {}
    
    /// value : True = Play, False = Pause
    pub fn set_playback(&self, value: bool) {}

    /// value : True = shuffle, False = Don't Shuffle
    pub fn set_shuffle(&self, value: bool) {}

    // dir : either Next or Previous
    pub fn skip_playback(&self, dir: SkipDirection) {}
}