use crate::db;
use crate::users::AuthenticatedUser;
use db::LinkUse;
use rocket::Rocket;
use rocket_contrib::json::Json;
use serde::Serialize;
use std::net::Ipv4Addr;

#[derive(Serialize)]
struct Response {
    status: u32,
    link_id: Option<String>,
    link_url: Option<String>,
    tracks: Option<Vec<LinkTrack>>,
}

#[derive(Serialize)]
struct LinkTrack {
    pub ip: String,
    pub user_agent: String,
    pub time: String,
}

impl LinkTrack {
    fn from_db(l: db::LinkUse) -> Self {
        let mut ip_str = String::from("N/A");
        if let Some(ip) = l.ip {
            let mut bytes: [u8; 4] = [0; 4];
            bytes.copy_from_slice(ip.as_slice());
            let addr = Ipv4Addr::from(bytes);
            ip_str = addr.to_string();
        }

        Self {
            ip: ip_str,
            user_agent: l.user_agent,
            time: l.ts.to_string(),
        }
    }
}

#[get("/?<id>")]
fn track(id: String, connection: db::DbConn, user: AuthenticatedUser) -> Json<Response> {
    if let Some((item, tracks)) = LinkUse::get_all_tracks(&id, user.id, &connection) {
        let tracks: Vec<LinkTrack> = tracks.into_iter().map(|t| LinkTrack::from_db(t)).collect();
        Json(Response {
            status: 200,
            link_id: Some(item.id),
            link_url: Some(item.url),
            tracks: Some(tracks),
        })
    } else {
        Json(Response {
            status: 404,
            link_id: None,
            link_url: None,
            tracks: None,
        })
    }
}

pub fn mount(rocket: Rocket) -> Rocket {
    rocket.mount("/api/track", routes![track])
}
