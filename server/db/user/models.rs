use super::schema::*;
use diesel::prelude::*;
use diesel::{Insertable, PgConnection};
use serde::{Deserialize, Serialize};

#[derive(Insertable, Deserialize, Serialize, FromForm)]
#[table_name = "users"]
pub struct User {
    pub username: String,
}

#[derive(Queryable, Serialize)]
pub struct UserEntity {
    pub id: i32,
    pub username: String,
}

impl User {
    pub fn create(username: &str, conn: &PgConnection) -> UserEntity {
        diesel::insert_into(users::table)
            .values(users::dsl::username.eq(username))
            .get_result(conn)
            .expect("Failed to insert user!")
    }

    pub fn get_by_username(username_: &str, conn: &PgConnection) -> Option<UserEntity> {
        use super::schema::users::dsl::*;
        users
            .filter(username.eq(username_))
            .first::<UserEntity>(conn)
            .ok()
    }
}

#[derive(Insertable)]
pub struct AuthInfo {
    pub user_id: i32,
    pub password_hash: String,
}

#[derive(Queryable)]
pub struct AuthInfoEntity {
    pub id: i32,
    pub user_id: i32,
    pub password_hash: String,
}

impl AuthInfo {
    pub fn create(user_id: i32, password_hash: &str, conn: &PgConnection) -> AuthInfoEntity {
        diesel::insert_into(auth_infos::table)
            .values((
                auth_infos::dsl::user_id.eq(user_id),
                auth_infos::dsl::password_hash.eq(password_hash),
            ))
            .get_result(conn)
            .expect("Failed to insert auth info!")
    }

    pub fn get_by_user_id(user_id_: i32, conn: &PgConnection) -> Option<AuthInfoEntity> {
        use super::schema::auth_infos::dsl::*;
        auth_infos
            .filter(user_id.eq(user_id_))
            .first::<AuthInfoEntity>(conn)
            .ok()
    }
}
