#[macro_use]
extern crate rocket;

use std::fs;
use std::io;
use std::str;

mod cors;

#[get("/")]
fn index() -> &'static str {
    "Nothing to see here..."
}

#[get("/alive")]
fn alive() -> &'static str {
    "Backend Alive"
}

#[get("/?<code>&<state>")]
fn code_extraction(code: &str, state: &str) -> io::Result<&'static str> {
    // Check to see if the state is a match
    let file_bytes = fs::read("state")?;
    let current_state: &str = str::from_utf8(&file_bytes).unwrap();

    if current_state == state {
        // Write the code retrieved from the query to the code file
        fs::write("code", code)?;
        Ok("Login Successful!")
    } else {
        Ok("Error: State Invalid!")
    }
}

#[launch]
fn rocket() -> _ {
    let config = rocket::Config::figment()
                    .merge(("port", 5000));
    rocket::custom(config)
        .attach(cors::CORS)
        .mount("/", routes![index, alive, code_extraction])
        
}
