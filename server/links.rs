use crate::db::{self, LinkItem};
use crate::users::AuthenticatedUser;
use rocket::Rocket;
use rocket_contrib::json::Json;
use serde::Serialize;

#[derive(Serialize)]
struct Response {
    links: Option<Vec<LinkItem>>,
}

#[get("/all")]
fn all(connnection: db::DbConn, user: AuthenticatedUser) -> Json<Response> {
    Json(Response {
        links: LinkItem::get_all_user(user.id, &connnection),
    })
}

pub fn mount(rocket: Rocket) -> Rocket {
    rocket.mount("/api/links", routes![all])
}
