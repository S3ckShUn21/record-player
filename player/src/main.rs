// mod spotify;

// use dotenv;
// use open;
// use reqwest;
// use std::error::Error;
use std::{thread, time};

use spidev::{Spidev, SpidevOptions, SpiModeFlags};
use std::io;

use rfid_rs;
use rfid_rs::picc;

fn create_spi() -> io::Result<Spidev> {
    let mut spi = Spidev::open("/dev/spidev0.0")?;
    let options = SpidevOptions::new()
        .bits_per_word(8)
        .max_speed_hz(20_000)
        .mode(SpiModeFlags::SPI_MODE_0)
        .build();
    spi.configure(&options)?;
    Ok(spi)
}

fn main() {
    let spi = create_spi().unwrap();
    let mut reader = rfid_rs::MFRC522 { spi };
    reader.init().expect("Reader Initialization Failed!");

    loop {
        let found_card = reader.new_card_present().is_ok();

        if found_card {

            thread::sleep( time::Duration::from_millis(500) );
 
            // Get the card's UID
            let uid = match reader.read_card_serial() {
                Ok(u) => u,
                Err(e) => {
                    println!("Could not read card: {:?}", e);
                    continue
                },
            };
            println!("Found a new card : {:02X?}", uid);
            
            
            let key: rfid_rs::MifareKey = [0xffu8; 6];
            let len = 18;

            // Attempt to read all the blocks in the card 
            for block in 0..64 {

                // Authenticate the block so we can read data from it
                // Technically we must only authenticate the sector (i.e. once every 4 blocks) which would save some time
                match reader.authenticate(picc::Command::MfAuthKeyA, block, key, &uid) {
                    Ok(_) => (),
                    Err(e) => {
                        println!("Could not authenticate card {:?}", e);
                        break
                    }
                }
    
                // Attempt to read data from the block
                match reader.mifare_read(block, len) {
                    Ok(response) => println!("Read block {:02}: {:02X?}", block, response.data),
                    Err(e) => {
                        println!("Failed reading block {}: {:?}", block, e);
                        break
                    }
                }

            }


            reader.halt_a().expect("Could not halt");
            reader.stop_crypto1().expect("Could not stop crypto1");

        }
    }
}

/* Block out this code for right now so I can test the RPI peripheral stuff

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let s = spotify::Spotify::new();
    s.test();

    // These are the query paramters for the api get request
    let client_id = dotenv::var("CLIENT_ID").unwrap();
    let params = [
        ("client_id", client_id.as_str()),
        ("response_type", "code"),
        ("redirect_uri", "http://localhost/"),
        ("scope", "user-read-private user-read-email")
    ];
    // This is the url for the get request
    let url = "https://accounts.spotify.com/authorize";

    let client = reqwest::Client::new(); // This is the "client" to use to send GET/POST/etc.

    let req = client
                .get(url)       // Make it a GET req
                .query(&params) // Use the given params in the query
                .build()        // Build the request but don't actually send it to spotify
                .unwrap();      
    
    println!("{:?}", req);      // Debug print

    // Actually send the request and await it's resposne back
    // The response's URL is the page to go to, at which point the USER CREDENTIALS (user and pass) get entered
    let res = client.execute(req).await?; 

    // Debug print the response
    println!("{:?}", res);

    // Actually GO to the page that the response gave back so the user can enter their creds
    open::that(res.url().as_str()).unwrap();

    Ok(())
}

*/

// **** STEP 1 ****
// This what was done above in the code

// **** STEP 2 ****
// This is the redirected uri that I get after "open::that" with the response url runs
// It takes me to the authentication screen, I enter credentials and accept, and then the page redirects to this (below)
// http://localhost/?code=AQDNll_zu9GH8NwjyOKvOO2rOfYRWzdGM-XupHe-T2DVWx_c1kQot62JXq85bkVdTuVXIxs7Cj4InwIf-6sBffFDS1BbsXYrsVirxAJCNJOEoYwtwsIc2eUGphU6UpRoZ_l1LgnBopFH6M1Yb91tVtiB8qilq5fXbqxDzHisQyxzlU0Ejqyg6jf4w4H3CVTQGDedDlkkVkMY
// THEN, I assume, I can have something listening on "http://localhost/" or "http://localhost/spotify-callback/" or whatever
// And when this gets sent to local host, I can extract the code given in the query section

// **** STEP 3 ****
// THENNNNNNN
// The code extracted above gets sent within a POST request ( to a different endpoint ) 
// so I can actually get an ACCESS TOKEN ( & a refresh token )

// NOTE : The POST request requires params to be in the BODY of the request not in the QUERY
//        Also, some specific headers are additionally required

// The response of the POST request will have JSON like the following
// {
//     "access_token": "NgCXRK...MzYjw",
//     "token_type": "Bearer",
//     "scope": "user-read-private user-read-email",
//     "expires_in": 3600,
//     "refresh_token": "NgAagA...Um_SHo"
//  }

// **** STEP 4 ****
// LASTLY
// If the access token is expired, then I can use the refresh token to get a new one at the same endpoint as STEP 3
// The POST request body is a bit different but, all in all similar
// The refresh token will be used over and over again to get new access tokens