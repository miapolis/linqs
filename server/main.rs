#![feature(decl_macro)]
#![feature(stmt_expr_attributes)]

use rocket::http::hyper::header::UserAgent;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::response::NamedFile;
use rocket_contrib::json;
use rocket_contrib::json::JsonValue;
use std::net::{IpAddr, Ipv4Addr};

// Included to prevent linker errors
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
mod create;
mod db;
mod links;
mod password;
mod redirect;
mod serve_static;
mod track;
mod users;

use config::Config;

lazy_static! {
    pub static ref CONFIG: Config = Config::create();
}

#[derive(Debug)]
struct UserData {
    ip: Option<Ipv4Addr>,
    user_agent: UserAgent,
}

impl<'a, 'r> FromRequest<'a, 'r> for UserData {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let client_ip = request.client_ip().unwrap();
        let ip_addr = match client_ip {
            IpAddr::V4(addr) => Some(addr),
            _ => None,
        };
        let user_agent = UserAgent(request.headers().get_one("User-Agent").unwrap().to_string());

        Outcome::Success(Self {
            ip: ip_addr,
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
    rocket = create::mount(rocket);
    rocket = track::mount(rocket);
    rocket = links::mount(rocket);
    rocket = users::mount(rocket);

    if cfg!(debug_assertions) {
        // Use CORS only in debug mode
        rocket = rocket.attach(cors::CORS);
    }

    rocket.register(catchers![unauthorized, not_found]).launch();
}
