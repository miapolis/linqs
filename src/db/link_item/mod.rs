mod models;
mod schema;
use diesel::PgConnection;
pub use models::LinkItem;
pub use models::LinkUse;
use nanoid::nanoid;

pub fn create_link(path: &str, url: &str, conn: &PgConnection) -> LinkItem {
    let id;
    if path.is_empty() {
        id = nanoid!(6);
    } else {
        id = path.to_owned();
    }
    LinkItem::create(&id, url, &nanoid!(10), conn)
}
