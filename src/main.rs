#![feature(decl_macro)]

use rocket::http::hyper::header::UserAgent;
use rocket::request::{FromRequest, Outcome, Request};
use std::net::{IpAddr, Ipv4Addr};

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

mod cors;
mod create;
mod db;
mod redirect;
mod serve_static;
mod track;

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

fn main() {
    dotenv::dotenv().ok();

    let mut rocket = rocket::ignite().manage(db::init_pool());
    rocket = serve_static::mount(rocket);
    rocket = redirect::mount(rocket);
    rocket = create::mount(rocket);
    rocket = track::mount(rocket);
    rocket.launch();
}
