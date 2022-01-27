#[macro_use] extern crate rocket;

use rocket::response::content::Html;

#[get("/")]
fn index() -> Html<&'static str> {
    Html(
    r#"
    <html>
        <body> 
            Hello from Raspberry Pi
            <a href="http://youtube.com">
                <button>Click Me</button>
            </a>
        </body>
    </html>
    "#
    )
}

#[launch]
fn rocket() -> _ {

    let config = rocket::Config::figment()
                    .merge(("port", 8080))
                    .merge(("address", "0.0.0.0"));

    rocket::custom(config)
                    .mount("/", routes![index])
}