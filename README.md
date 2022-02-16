# Record Player

This project is my modern take on the vinyl record player. I'm also using this project to teach myself the [Rust programming language](https://doc.rust-lang.org/book/).

### Overarching Goal

> Create a product that mimics a vinyl record player using modern technology

### Minimum Viable Product (MVP)

- [ ] A self enclosed (headless) prodcut that could be displayed on furniture
- [ ] The ability to play a specific song, album, or playlist
- [ ] Auxilery sound output (to other speakers)
- [ ] Physical pause, play, skip buttons
- [ ] The ability to "program" the "records" easily

### Implmenetation Details

* The brains of the record player is a Raspberry Pi 3B+
* I'm currently using MIFARE RFID cards along with an [MFRC522](https://www.amazon.com/mfrc522/s?k=mfrc522) card reader for the "records"
* I'm using the [Spotify API](https://developer.spotify.com/documentation/web-api/) to play the music 
* The RFID cards will contain spotify URI data along with any additional API query JSON necessary for that specific record
* Because I'm requiring the product be headless, I've deceided to create a frontend web user interface to interact with the device

### Roadmap

- [X] Setup a Rust environment & get to grips with the basics of Rust
- [X] Figure out how the Spotify API works and what I can accomplish with it.
- [X] Figure out how Spotify's Auth system works
- [ ] Create a streamlined system to authorize the record player
- [X] Understand how the RFID cards work
- [ ] Finalize the RFID data serialization format
- [ ] Build out a Spotify API library that can be used to access the API easily 
- [ ] Build a basic front end to allow the user to login with their spotify account. Have this complete the authorization process & cache tokens
- [ ] Create a backend that can handle the spotify redirects, and any other hardware specific configuration I want to acheive from the front end
    * Set the WiFi creds, login to spotify, etc.
- [ ] Create a way to program the RFID cards from the frontend
- [ ] ???
- [ ] Profit

### Current Design Questions

* Should I use javascript and react for my frontend or stick with rust's Yew framework
* Why is the Web so complicated?!?

### Post MVP Product Ideas

* A frontend app that will automatically design the "record" art that can be applied to the RFID card ( album art, title, spotify waveform, etc.)
* Ability to play an artist shuffle, the most recent episode of a podcast, random episode of a podcast, etc.
* Internal speakers
* More spotify specific physical buttons (shuffle, replay playlist, replay single, like)
* Light bar for an indicator ( playback duration length, error status, notifications, etc.)
* Have the Raspberry Pi host a wifi network that the user connects to initially, so they can enter their personal wifi creds. Then they'll be able to access the record player over their network 