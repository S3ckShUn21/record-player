mod spotify;

use dotenv;
use open;
use reqwest;
use std::error::Error;

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