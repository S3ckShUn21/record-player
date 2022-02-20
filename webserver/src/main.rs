#[macro_use]
extern crate rocket;

use std::{str, path::PathBuf};

use dotenv;
use rocket::serde::json::{Json, json, Value};
use rocket::response::Redirect;

mod environment;
use environment::SpotifyAuthEnvVars;
mod cors;
use cors::CORS;

use spotify_lib::{self, SpotifyApi, get_access_token_response};

#[get("/")]
fn index() -> &'static str {
    "Nothing to see here..."
}

#[get("/alive")]
fn alive() -> &'static str {
    "alive"
}

#[get("/env")]
fn environment_vars() -> Json<SpotifyAuthEnvVars> {
    // Fill in the details for the env data
    let env_vars = SpotifyAuthEnvVars {
        client_id: dotenv::var("CLIENT_ID").unwrap(),
        scope: dotenv::var("SCOPE").unwrap(),
        redirect_uri: dotenv::var("REDIRECT_URI").unwrap(),
    };
    // Send the data as json
    Json(env_vars)
}

#[get("/user")]
fn is_logged_in() -> Value {
    let cache_path = PathBuf::from( dotenv::var("CACHE_FILE").unwrap() );
    match SpotifyApi::read(&cache_path) {
        Ok(mut h) => {
            let user_name = h.get_user_name().unwrap();
            return json!({
                "logged_in" : true,
                "user_name" : user_name,
            });
        },
        Err(e) => {
            println!("{:?}", e);
            return json!({
                "logged_in" : false,
                "user_name" : "",
            });
        }
    };
}

#[get("/?<code>&<state>")]
fn code_extraction(code: &str, state: &str) -> Redirect {
    
    let token_data = get_access_token_response(code).unwrap();
    let cache_path = PathBuf::from( dotenv::var("CACHE_FILE").unwrap() );

    let handler = SpotifyApi::from_token_data(&cache_path, token_data);
    handler.cache().unwrap();
    
    Redirect::to("http://localhost:3000/")
}

#[launch]
fn rocket() -> _ {
    // Load the environment variables
    dotenv::dotenv().ok();

    let config = rocket::Config::figment()
                    .merge(("port", 5000));
    rocket::custom(config)
        .attach(CORS)
        .mount("/", routes![index, alive, environment_vars, is_logged_in, code_extraction])
        
}
