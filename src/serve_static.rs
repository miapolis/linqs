use rocket::Rocket;
use rocket_contrib::serve::StaticFiles;

pub fn mount(rocket: Rocket) -> Rocket {
    let rocket = rocket.mount("/", StaticFiles::from("./frontend/dist"));
    rocket.mount("/track", StaticFiles::from("./frontend/dist/track").rank(1))
}
