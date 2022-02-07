use crate::db;
use crate::path::VALID_LINK_REGEX;
use crate::users::AuthenticatedUser;
use db::create_link;
use db::{LinkItem, TrackItem};
use rocket::Rocket;
use rocket_contrib::json;
use rocket_contrib::json::Json;
use rocket_contrib::json::JsonValue;
use serde::Deserialize;
use url::Url;

#[derive(Deserialize)]
struct CreateRequest {
    path: Option<String>,
    url: String,
    track_time: bool,
    track_ip: bool,
    track_user_agent: bool,
}

#[post("/create", data = "<req>")]
fn submit(req: Json<CreateRequest>, connection: db::DbConn, user: AuthenticatedUser) -> JsonValue {
    if Url::parse(&req.url).is_err() {
        return json!({
            "status": "error",
            "message": "url is invalid"
        });
    }

    if req.path.is_some() && !VALID_LINK_REGEX.is_match(&req.path.as_ref().unwrap()) {
        return json!({
            "status": "error",
            "message": "path is invalid"
        });
    }

    if req.path.is_some() && LinkItem::get_id(&req.path.as_ref().unwrap(), &connection).is_some() {
        return json!({
            "status": "error",
            "message": "path already exists",
        });
    }

    let mut to_track = Vec::new();
    if req.track_time {
        to_track.push(TrackItem::Time);
    }
    if req.track_ip {
        to_track.push(TrackItem::Ip);
    }
    if req.track_user_agent {
        to_track.push(TrackItem::UserAgent);
    }

    let item = create_link(
        user.id,
        req.path.as_ref().and_then(|p| Some(p.as_str())),
        &req.url,
        to_track,
        &connection,
    );

    json!({
        "status": "ok",
        "id": item.id,
        "track_id": item.track_id
    })
}

pub fn mount(rocket: Rocket) -> Rocket {
    rocket.mount("/api/", routes![submit])
}
