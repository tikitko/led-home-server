#[macro_use]
extern crate rocket;

use magic_home_rs::*;

use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::{Header, Status};
use rocket::serde::{json::Json, Serialize};
use rocket::{Request, Response};

const IP: &'static str = "192.168.1.160:5577";

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }
    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, PATCH, PUT, DELETE, HEAD, OPTIONS, GET",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

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
        Err(_) => Status::InternalServerError,
    }
}

#[get("/state")]
fn state() -> Result<Json<StateResponse>, Status> {
    magic_home()
        .state()
        .map(|state| {
            Json(StateResponse {
                is_enabled: state.is_enabled,
                red: state.red,
                green: state.green,
                blue: state.blue,
            })
        })
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
        .attach(CORS)
        .mount("/", routes![state, power, color])
}
