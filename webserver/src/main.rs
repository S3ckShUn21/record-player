#[macro_use]
extern crate rocket;

use std::{str};

use dotenv;
use rocket::serde::json::Json;
use rocket::response::Redirect;

mod environment;
use environment::SpotifyAuthEnvVars;
mod cors;
use cors::CORS;

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
    // Load the environment variables
    dotenv::dotenv().ok();
    // Fill in the details for the env data
    let env_vars = SpotifyAuthEnvVars {
        client_id: dotenv::var("CLIENT_ID").unwrap(),
        scope: dotenv::var("SCOPE").unwrap(),
        redirect_uri: dotenv::var("REDIRECT_URI").unwrap(),
    };
    // Send the data as json
    Json(env_vars)
}

#[get("/?<code>&<state>")]
fn code_extraction(code: &str, state: &str) -> Redirect {
    println!("Code: {}", code);
    println!("State: {}", state);
    Redirect::to("http://localhost:3000/")
}

#[launch]
fn rocket() -> _ {
    let config = rocket::Config::figment()
                    .merge(("port", 5000));
    rocket::custom(config)
        .attach(CORS)
        .mount("/", routes![index, alive, environment_vars, code_extraction])
        
}
