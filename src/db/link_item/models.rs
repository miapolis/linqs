use super::schema::{link_items, link_uses};
use super::schema::{link_items::dsl as li_dsl, link_uses::dsl as lu_dsl};
use chrono::Utc;
use diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::BelongingToDsl;
use diesel::RunQueryDsl;
use std::net::Ipv4Addr;

#[derive(Identifiable, Queryable, AsChangeset, Debug)]
#[table_name = "link_items"]
pub struct LinkItem {
    pub id: String,
    pub url: String,
    pub track_id: String,
}

#[derive(Insertable)]
#[table_name = "link_items"]
pub struct InsertableLinkItem {
    pub id: String,
    pub url: String,
    pub track_id: String,
}

#[derive(Identifiable, Queryable, Debug, Associations)]
#[belongs_to(LinkItem)]
#[table_name = "link_uses"]
pub struct LinkUse {
    pub id: i32,
    pub link_item_id: String,
    pub ip: Option<Vec<u8>>,
    pub user_agent: String,
    pub ts: chrono::NaiveDateTime,
}

impl LinkItem {
    pub fn create(id: &str, url: &str, track_id: &str, conn: &PgConnection) -> LinkItem {
        diesel::insert_into(link_items::table)
            .values(&InsertableLinkItem {
                id: id.to_owned(),
                url: url.to_owned(),
                track_id: track_id.to_owned(),
            })
            .get_result(conn)
            .expect("Failed to insert link item!")
    }

    pub fn get_id(id: &str, conn: &PgConnection) -> Option<LinkItem> {
        li_dsl::link_items.find(id).first::<LinkItem>(conn).ok()
    }

    pub fn get_track(id: &str, conn: &PgConnection) -> Option<LinkItem> {
        li_dsl::link_items
            .filter(li_dsl::track_id.eq(id))
            .first::<LinkItem>(conn)
            .ok()
    }

    pub fn consume(
        id: &str,
        ip: Option<Ipv4Addr>,
        user_agent: &str,
        conn: &PgConnection,
    ) -> Option<LinkItem> {
        println!("ID: {:?}", id);
        if let Some(item) = Self::get_id(id, conn) {
            let bytes = ip.and_then(|i| Some(i.octets().to_vec()));
            diesel::insert_into(link_uses::table)
                .values((
                    lu_dsl::link_item_id.eq(item.id.clone()),
                    lu_dsl::ip.eq(bytes),
                    lu_dsl::user_agent.eq(user_agent),
                    lu_dsl::ts.eq(Utc::now().naive_utc()),
                ))
                .execute(conn)
                .expect("Failed to insert link use!");

            Some(item)
        } else {
            None
        }
    }
}

impl LinkUse {
    pub fn get_all_tracks(id: &str, conn: &PgConnection) -> Option<(LinkItem, Vec<Self>)> {
        if let Some(item) = LinkItem::get_track(id, conn) {
            if let Some(tracks) = Self::belonging_to(&item).load::<LinkUse>(conn).ok() {
                return Some((item, tracks));
            }
        }

        None
    }
}
