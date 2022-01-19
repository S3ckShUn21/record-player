mod spotify {

    pub enum SkipDirection {
        Next,
        Prev
    }
    
    /// Switch the playback to this URI immediately on the currently active device
    pub fn play_uri(uri: &str) {}
    
    /// value : True = Play, False = Pause
    pub fn set_playback(value: bool) {}

    /// value : True = shuffle, False = Don't Shuffle
    pub fn set_shuffle(value: bool) {}

    // dir : either Next or Previous
    pub fn skip_playback(dir: SkipDirection) {}

}