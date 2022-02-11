mod models;
mod schema;
use diesel::PgConnection;
pub use models::LinkItem;
pub use models::LinkUse;
pub use models::TrackItem;
use nanoid::nanoid;

pub fn create_link(
    user_id: i32,
    path: Option<&str>,
    url: &str,
    to_track: Vec<TrackItem>,
    max_uses: Option<i32>,
    expires_at_minutes: Option<i64>,
    conn: &PgConnection,
) -> LinkItem {
    let id;
    if path.is_none() {
        id = nanoid!(6);
    } else {
        id = path.unwrap().to_owned();
    }

    let expires_at = if let Some(expires_at_minutes) = expires_at_minutes {
        Some(chrono::Utc::now().naive_utc() + chrono::Duration::minutes(expires_at_minutes))
    } else {
        None
    };

    LinkItem::create(
        &id,
        user_id,
        url,
        &nanoid!(10),
        expires_at,
        max_uses,
        to_track,
        conn,
    )
}
