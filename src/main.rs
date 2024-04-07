#[macro_use]
extern crate rocket;

use magic_home_rs::*;

use rocket::http::Status;
use rocket::serde::{Serialize, json::Json};

const IP: &'static str = "192.168.1.160:5577";

fn magic_home() -> MagicHome {
    let mut magic_home = MagicHome::new();
    magic_home.connect(IP).unwrap();
    magic_home
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct StateResponse {
    is_enabled: bool,
    red: u8,
    green: u8,
    blue: u8,
}

fn to_status<T, E>(result: Result<T, E>) -> Status {
    match result {
        Ok(_) => Status::Ok,
        Err(_) => Status::InternalServerError
    }
}

#[get("/state")]
fn state() -> Result<Json<StateResponse>, Status> {
    magic_home().state()
        .map(|state| Json(StateResponse {
            is_enabled: state.is_enabled,
            red: state.red,
            green: state.green,
            blue: state.blue,
        }))
        .map_err(|_| Status::InternalServerError)
}

#[put("/power/<value>")]
fn power(value: bool) -> Status {
    to_status(magic_home().power(value))
}

#[put("/color/<red>/<green>/<blue>")]
fn color(red: u8, green: u8, blue: u8) -> Status {
    to_status(magic_home().set_color([red, green, blue]))
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![state, power, color])
}
