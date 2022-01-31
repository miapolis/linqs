use crate::{db};
use db::LinkItem;
use db::create_link;
use rocket::Rocket;
use rocket_contrib::json;
use rocket_contrib::json::JsonValue;
use url::Url;

#[post("/create?<path>&<url>")]
fn submit(path: String, url: String, connection: db::DbConn) -> JsonValue {
    if url.is_empty() {
        return json!({
            "status": "error",
            "message": "url is required"
        });
    }

    if Url::parse(&url).is_err() {
        return json!({
            "status": "error",
            "message": "url is invalid"
        });
    }

    if !path.is_empty() && LinkItem::get_id(&path, &connection).is_some() {
        return json!({
            "status": "error",
            "message": "path already exists",
        });
    }

    let item = create_link(&path, &url, &connection);

    json!({
        "status": "ok",
        "id": item.id,
        "track_id": item.track_id
    })
}


pub fn mount(rocket: Rocket) -> Rocket {
    rocket.mount("/api/", routes![submit])
}
