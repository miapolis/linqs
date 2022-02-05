use crate::db;
use crate::users::AuthenticatedUser;
use db::LinkUse;
use rocket::Rocket;
use rocket_contrib::json::Json;
use serde::Serialize;

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
        Self {
            ip: l.ip,
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
