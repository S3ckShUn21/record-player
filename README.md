# Record Player

This project is my modern take on the vinyl record player. I'm also using this project to teach myself the [Rust programming language](https://doc.rust-lang.org/book/).

### Overarching Goal

> Create a product that mimics a vinyl record player using modern technology

### Minimum Viable Product (MVP)

- [ ] A self enclosed (headless) product that could be displayed on furniture
- [ ] The ability to play a specific song, album, or playlist
- [ ] Auxiliary sound output (to other speakers)
- [ ] Physical pause, play, skip buttons
- [ ] The ability to "program" the "records" easily


### Post MVP Product Ideas

* A frontend app that will automatically design the "record" art that can be applied to the RFID card ( album art, title, spotify waveform, etc.)
* Ability to play an artist shuffle, the most recent episode of a podcast, random episode of a podcast, etc.
* Internal speakers
* More spotify specific physical buttons (shuffle, replay playlist, replay single, like)
* Light bar for an indicator ( playback duration length, error status, notifications, etc.)
* Have the Raspberry Pi host a wifi network that the user connects to initially, so they can enter their personal wifi creds. Then they'll be able to access the record player over their network 

## Version 0.2.0

> Sophisticated does ***NOT*** inherently mean complicated

Switching to python for the backend & firmware. Keeping JS for the frontend application.

* Backend Framework => Flask (Python)
* There are SPI and RFID modules for Python