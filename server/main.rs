#![feature(decl_macro)]
#![feature(stmt_expr_attributes)]

use rocket::http::hyper::header::UserAgent;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::response::NamedFile;
use rocket_contrib::json;
use rocket_contrib::json::JsonValue;

// Include OpenSSL to prevent linker errors
extern crate openssl;
#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_migrations;
#[macro_use]
extern crate lazy_static;

mod config;
mod cors;
mod db;
mod links;
mod password;
mod path;
mod redirect;
mod serve_static;
mod users;

use config::Config;

lazy_static! {
    pub static ref CONFIG: Config = Config::create();
}

#[derive(Debug)]
struct UserData {
    ip: String,
    user_agent: UserAgent,
}

impl<'a, 'r> FromRequest<'a, 'r> for UserData {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let client_ip = format!("{}", request.client_ip().unwrap());
        let user_agent = UserAgent(request.headers().get_one("User-Agent").unwrap().to_string());

        Outcome::Success(Self {
            ip: client_ip,
            user_agent,
        })
    }
}

#[catch(401)]
fn unauthorized() -> JsonValue {
    json!({
        "error": "unauthorized",
    })
}

#[catch(404)]
fn not_found() -> NamedFile {
    NamedFile::open("./frontend/dist/404.html").unwrap()
}

fn main() {
    dotenv::dotenv().ok();
    CONFIG.print();

    let mut rocket = rocket::ignite().manage(db::init_pool());
    rocket = serve_static::mount(rocket);
    rocket = redirect::mount(rocket);
    rocket = users::mount(rocket);
    rocket = links::mount(rocket);

    if cfg!(debug_assertions) {
        // Use CORS only in debug mode
        rocket = rocket.attach(cors::CORS);
    }

    rocket.register(catchers![unauthorized, not_found]).launch();
}
