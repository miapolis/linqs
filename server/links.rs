use crate::db::create_link;
use crate::db::LinkUse;
use crate::db::{self, LinkItem, TrackItem};
use crate::path;
use crate::users::AuthenticatedUser;
use rocket::http::Status;
use rocket::Rocket;
use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};
use serde::{Deserialize, Serialize};
use url::Url;

const RESERVED_PATHS: [&'static str; 1] = ["link"];

#[derive(Deserialize)]
struct CreateRequest {
    path: Option<String>,
    url: String,
    track_time: bool,
    track_ip: bool,
    track_user_agent: bool,
}

#[post("/create", data = "<req>")]
fn create(req: Json<CreateRequest>, connection: db::DbConn, user: AuthenticatedUser) -> JsonValue {
    if Url::parse(&req.url).is_err() {
        return json!({
            "status": "error",
            "message": "url is invalid"
        });
    }

    if req.path.is_some() && !path::valid(&req.path.as_ref().unwrap()) {
        return json!({
            "status": "error",
            "message": "path is invalid"
        });
    }

    if req.path.is_some() && RESERVED_PATHS.contains(&req.path.as_ref().unwrap().as_str()) {
        return json!({
            "status": "error",
            "message": "path is reserved"
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

#[derive(Serialize)]
struct GetResponse {
    status: u32,
    link_id: Option<String>,
    link_url: Option<String>,
    fields: Option<Vec<TrackItem>>,
    uses: Option<i32>,
    tracks: Option<Vec<LinkTrack>>,
}

#[derive(Serialize)]
struct LinkTrack {
    pub ip: Option<String>,
    pub user_agent: Option<String>,
    pub time: Option<String>,
}

impl LinkTrack {
    fn from_db(l: db::LinkUse) -> Self {
        Self {
            ip: l.ip,
            user_agent: l.user_agent,
            time: l.ts.and_then(|ts| Some(ts.to_string())),
        }
    }
}

#[get("/<id>")]
fn get(id: String, connection: db::DbConn, user: AuthenticatedUser) -> Json<GetResponse> {
    if let Some((item, tracks)) = LinkUse::get_all_tracks(&id, user.id, &connection) {
        let tracks: Vec<LinkTrack> = tracks.into_iter().map(|t| LinkTrack::from_db(t)).collect();
        Json(GetResponse {
            status: 200,
            link_id: Some(item.id),
            link_url: Some(item.url),
            fields: Some(item.to_track),
            uses: Some(item.uses),
            tracks: Some(tracks),
        })
    } else {
        Json(GetResponse {
            status: 404,
            link_id: None,
            link_url: None,
            fields: None,
            uses: None,
            tracks: None,
        })
    }
}

#[delete("/<id>")]
fn delete(id: String, connection: db::DbConn, user: AuthenticatedUser) -> Status {
    if let Some(item) = LinkItem::get_track(&id, user.id, &connection) {
        LinkItem::delete(&item.id, &connection);
        return Status::NoContent;
    }
    Status::NotFound
}

#[options("/<_id>")]
#[cfg(debug_assertions)]
fn delete_opt(_id: String) -> Status {
    Status::Ok
}

#[derive(Serialize)]
struct AllResponse {
    links: Option<Vec<LinkItem>>,
}

#[get("/")]
fn all(connnection: db::DbConn, user: AuthenticatedUser) -> Json<AllResponse> {
    Json(AllResponse {
        links: LinkItem::get_all_user(user.id, &connnection),
    })
}

#[cfg(debug_assertions)]
pub fn mount(rocket: Rocket) -> Rocket {
    rocket.mount("/api/links", routes![create, get, delete, delete_opt, all])
}

#[cfg(not(debug_assertions))]
pub fn mount(rocket: Rocket) -> Rocket {
    rocket.mount("/api/links", routes![create, get, delete, all])
}
