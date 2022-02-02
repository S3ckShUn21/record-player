use dotenv;
use open;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use ureq;

use std::error::Error;
use std::fs;

/// Generates a random 32 character alphanumeric string
fn generate_state() -> String {
    thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect()
}

fn main() -> Result<(), Box<dyn Error>> {
    // .env client_id
    let client_id = dotenv::var("CLIENT_ID")?;

    // Put the new state in the state file
    let state = generate_state();
    fs::write("state", &state)?;

    // The query params for the get request
    let params = [
        ("client_id", client_id.as_str()),
        ("response_type", "code"),
        ("redirect_uri", "http://localhost:8000/"),
        ("state", &state.as_str()),
        ("scope", "user-modify-playback-state user-read-private"),
    ];

    // This is the url for the get request
    let url = "https://accounts.spotify.com/authorize";

    // Create the request
    let mut req = ureq::get(url);
    // Append the queries to the request
    for &(k, v) in &params {
        req = req.query(k, v);
    }

    let result = req.call()?;

    open::that(result.get_url())?;

    Ok(())
}

// **** STEP 1 **** [DONE]
// This what was done above in the code

// **** STEP 2 **** [DONE]
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
