use diesel::pg::PgConnection;
use diesel::r2d2::{self, ConnectionManager, PooledConnection};
use diesel_migrations::embed_migrations;
use rocket::{
    http::Status,
    outcome::Outcome,
    request::{self, FromRequest},
    Request, State,
};
use std::{env, ops::Deref};

mod link;
mod user;
pub use link::*;
pub use user::*;

embed_migrations!();

type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn init_pool() -> Pool {
    let manager = ConnectionManager::<PgConnection>::new(&database_url());
    let pool = Pool::new(manager).expect("Failed to create pool");
    run_db_migrations(&pool.get().unwrap()).unwrap();

    pool
}

fn database_url() -> String {
    env::var("DATABASE_URL").expect("DATABASE_URL must be set")
}

fn run_db_migrations(conn: &PgConnection) -> Result<(), String> {
    match embedded_migrations::run(&*conn) {
        Ok(()) => Ok(()),
        Err(e) => Err(format!("Failed to run database migrations: {:?}", e)),
    }
}

pub struct DbConn(pub PooledConnection<ConnectionManager<PgConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'_>) -> request::Outcome<DbConn, Self::Error> {
        let pool = request.guard::<State<Pool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

impl Deref for DbConn {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
