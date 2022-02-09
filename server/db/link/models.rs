use super::schema::{link_items, link_uses};
use super::schema::{link_items::dsl as li_dsl, link_uses::dsl as lu_dsl};
use crate::db::User;
use chrono::Utc;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::BelongingToDsl;
use diesel::RunQueryDsl;
use diesel_derive_enum::DbEnum;
use serde::Serialize;

#[derive(Serialize, Debug, PartialEq, DbEnum)]
#[DieselType = "TrackItemMapping"]
pub enum TrackItem {
    Time,
    Ip,
    UserAgent,
}

#[derive(Serialize, Identifiable, Queryable, Debug, Associations)]
#[belongs_to(User)]
#[table_name = "link_items"]
pub struct LinkItem {
    pub id: String,
    #[serde(skip)]
    pub user_id: i32,
    pub url: String,
    pub track_id: String,
    pub uses: i32,
    pub to_track: Vec<TrackItem>,
}

#[derive(Insertable)]
#[table_name = "link_items"]
pub struct InsertableLinkItem {
    pub id: String,
    pub user_id: i32,
    pub url: String,
    pub track_id: String,
    pub uses: i32,
    pub to_track: Vec<TrackItem>,
}

#[derive(Identifiable, Queryable, Debug, Associations)]
#[belongs_to(LinkItem)]
#[table_name = "link_uses"]
pub struct LinkUse {
    pub id: i32,
    pub link_item_id: String,
    pub ip: Option<String>,
    pub user_agent: Option<String>,
    pub ts: Option<chrono::NaiveDateTime>,
}

impl LinkItem {
    pub fn create(
        id: &str,
        user_id: i32,
        url: &str,
        track_id: &str,
        to_track: Vec<TrackItem>,
        conn: &PgConnection,
    ) -> LinkItem {
        diesel::insert_into(link_items::table)
            .values(&InsertableLinkItem {
                id: id.to_owned(),
                user_id,
                url: url.to_owned(),
                track_id: track_id.to_owned(),
                uses: 0,
                to_track,
            })
            .get_result(conn)
            .expect("Failed to insert link item!")
    }

    pub fn get_id(id: &str, conn: &PgConnection) -> Option<LinkItem> {
        li_dsl::link_items.find(id).first::<LinkItem>(conn).ok()
    }

    pub fn get_all_user(user_id_: i32, conn: &PgConnection) -> Option<Vec<LinkItem>> {
        li_dsl::link_items
            .filter(li_dsl::user_id.eq(user_id_))
            .load::<LinkItem>(conn)
            .ok()
    }

    pub fn get_track(id: &str, user_id_: i32, conn: &PgConnection) -> Option<LinkItem> {
        li_dsl::link_items
            .filter(li_dsl::track_id.eq(id))
            .filter(li_dsl::user_id.eq(user_id_))
            .first::<LinkItem>(conn)
            .ok()
    }

    pub fn increment_uses(link_item: &LinkItem, conn: &PgConnection) {
        diesel::update(link_item)
            .set(li_dsl::uses.eq(li_dsl::uses + 1))
            .execute(conn)
            .ok();
    }

    pub fn consume(id: &str, ip: &str, user_agent: &str, conn: &PgConnection) -> Option<LinkItem> {
        if let Some(item) = Self::get_id(id, conn) {
            let ip = track_value(&item.to_track, TrackItem::Ip, ip);
            let user_agent = track_value(&item.to_track, TrackItem::UserAgent, user_agent);
            let time = track_value(&item.to_track, TrackItem::Time, Utc::now().naive_utc());

            if ip.is_some() || user_agent.is_some() || time.is_some() {
                diesel::insert_into(link_uses::table)
                    .values((
                        lu_dsl::link_item_id.eq(item.id.clone()),
                        lu_dsl::ip.eq(ip),
                        lu_dsl::user_agent.eq(user_agent),
                        lu_dsl::ts.eq(time),
                    ))
                    .execute(conn)
                    .expect("Failed to insert link use!");
            }

            Self::increment_uses(&item, conn);

            Some(item)
        } else {
            None
        }
    }

    pub fn delete(id: &str, conn: &PgConnection) {
        diesel::delete(link_items::table.find(id))
            .execute(conn)
            .ok();
        diesel::delete(link_uses::table.filter(lu_dsl::link_item_id.eq(id)))
            .execute(conn)
            .ok();
    }
}

fn track_value<T>(tracks: &Vec<TrackItem>, item: TrackItem, value: T) -> Option<T> {
    if tracks.contains(&item) {
        Some(value)
    } else {
        None
    }
}

impl LinkUse {
    pub fn get_all_tracks(
        id: &str,
        user_id: i32,
        conn: &PgConnection,
    ) -> Option<(LinkItem, Vec<Self>)> {
        if let Some(item) = LinkItem::get_track(id, user_id, conn) {
            if let Some(tracks) = Self::belonging_to(&item).load::<LinkUse>(conn).ok() {
                return Some((item, tracks));
            }
        }

        None
    }
}
