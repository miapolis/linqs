use crate::db::{self, AuthInfo, User};
use crate::password;
use rocket::http::{Cookie, Cookies, SameSite, Status};
use rocket::request::{FromRequest, Outcome};
use rocket::{Request, Rocket};
use rocket_contrib::json;
use rocket_contrib::json::{Json, JsonValue};
use serde::{Deserialize, Serialize};

#[derive(FromForm, Deserialize)]
struct CreateInfo {
    username: String,
    password: String,
}

#[derive(FromForm, Deserialize)]
struct LoginInfo {
    username: String,
    password: String,
}

pub struct AuthenticatedUser {
    pub id: i32,
}

#[derive(Debug, Serialize)]
enum LoginError {
    #[serde(rename(serialize = "username_does_not_exist"))]
    UsernameDoesNotExist,
    #[serde(rename(serialize = "wrong_password"))]
    WrongPassword,
}

#[derive(Debug, Serialize)]
struct LoginResponse {
    error: Option<LoginError>,
    user_id: Option<i32>,
}

impl LoginResponse {
    fn err(error: LoginError) -> Self {
        Self {
            error: Some(error),
            user_id: None,
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for AuthenticatedUser {
    type Error = JsonValue;

    fn from_request(req: &'a Request<'r>) -> Outcome<AuthenticatedUser, Self::Error> {
        let cookie_opt = req.cookies().get_private("user_id");
        match cookie_opt {
            Some(c) => {
                let user_id = c.value().parse::<i32>().unwrap();
                Outcome::Success(AuthenticatedUser { id: user_id })
            }
            None => Outcome::Failure((
                Status::Unauthorized,
                json!({ "error": "not_authenticated" }),
            )),
        }
    }
}

#[get("/me")]
fn me(connection: db::DbConn, user: AuthenticatedUser) -> JsonValue {
    let user = User::get_id(user.id, &connection);
    json!({
        "id": user.id,
        "username": user.username,
    })
}

#[post("/create", format = "json", data = "<create_info>")]
fn create(create_info: Json<CreateInfo>, connection: db::DbConn) -> Json<Option<i32>> {
    if crate::CONFIG.options.new_accounts {
        let user = User::create(&create_info.username, &connection);
        let password_hash = password::hash(&create_info.password);
        AuthInfo::create(user.id, &password_hash, &connection);
        Json(Some(user.id))
    } else {
        Json(None)
    }
}

#[post("/login", format = "json", data = "<login_info>")]
fn login(
    login_info: Json<LoginInfo>,
    conn: db::DbConn,
    mut cookies: Cookies,
) -> Json<LoginResponse> {
    let user_opt = User::get_by_username(&login_info.username, &conn);
    match user_opt {
        Some(user) => {
            let auth_opt = AuthInfo::get_by_user_id(user.id, &conn);
            match auth_opt {
                Some(auth) => {
                    if password::verify(&login_info.password, &auth.password_hash) {
                        cookies.add_private(
                            Cookie::build("user_id", user.id.to_string())
                                .same_site(SameSite::None)
                                .secure(true)
                                .finish(),
                        );
                        Json(LoginResponse {
                            error: None,
                            user_id: Some(user.id),
                        })
                    } else {
                        Json(LoginResponse::err(LoginError::WrongPassword))
                    }
                }
                None => Json(LoginResponse::err(LoginError::UsernameDoesNotExist)),
            }
        }
        None => Json(LoginResponse::err(LoginError::UsernameDoesNotExist)),
    }
}

#[options("/login")]
fn login_options() -> Status {
    Status::Ok
}

#[post("/logout")]
pub fn logout(mut cookies: Cookies) {
    cookies.remove_private(Cookie::named("user_id"))
}

pub fn mount(rocket: Rocket) -> Rocket {
    rocket.mount(
        "/api/users",
        routes![me, create, login, login_options, logout],
    )
}
