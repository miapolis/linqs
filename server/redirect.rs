use crate::{db, UserData};
use db::LinkItem;
use rocket::http::Status;
use rocket::response::Redirect;
use rocket::Rocket;

#[get("/<path>", rank = 2)]
fn index_redirect(
    path: String,
    connection: db::DbConn,
    user_data: UserData,
) -> Result<Redirect, Status> {
    let ip_addr = user_data.ip;
    let user_agent: String = user_data.user_agent.as_str().chars().take(500).collect();

    if let Some(item) = LinkItem::consume(&path, &ip_addr, &user_agent, &connection) {
        Ok(Redirect::to(item.url))
    } else {
        Err(Status::NotFound)
    }
}

pub fn mount(rocket: Rocket) -> Rocket {
    rocket.mount("/", routes![index_redirect])
}
