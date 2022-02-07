mod models;
mod schema;
use diesel::PgConnection;
pub use models::LinkItem;
pub use models::LinkUse;
pub use models::TrackItem;
use nanoid::nanoid;

pub fn create_link(user_id: i32, path: Option<&str>, url: &str, to_track: Vec<TrackItem>, conn: &PgConnection) -> LinkItem {
    let id;
    if path.is_none() {
        id = nanoid!(6);
    } else {
        id = path.unwrap().to_owned();
    }
    LinkItem::create(&id, user_id, url, &nanoid!(10), to_track, conn)
}
