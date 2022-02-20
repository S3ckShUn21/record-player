use spotify_lib;

fn main() {
    let api_cache = std::path::PathBuf::from("cache/spotify_api.json");

    let mut handler = spotify_lib::SpotifyApi::read(&api_cache).unwrap();

    handler.pause_playback().unwrap();
}
