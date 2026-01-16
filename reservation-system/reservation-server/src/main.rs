#[macro_use] extern crate rocket;

mod booking_process;
mod db_manager;
//mod cred;

use rocket::serde::Deserialize;
use rocket::serde::json::Json;
use rocket_db_pools::{Connection, Database};
use rocket_dyn_templates::{Template, context};
use crate::booking_process::{end_booking, start_booking};
use crate::db_manager::{add_interface_to_db, add_device_to_db, Db};



#[derive(Deserialize, Debug)]
pub struct BookingData {
    devices: u8,
    _minutes: i32,
    team_name: String,
    public_key: String,
}

#[derive(Deserialize, Debug)]
pub struct DeviceData{
    mac_address: String,
    interface_port: String, // we probably don't need this for the ESP32
    interface_id: String,
}

#[derive(Deserialize, Debug)]
pub struct InterfaceData{
    interface_id: String,
    ip_address: String,
    port: String,
    host_public_key: String,
    available: bool,
}

#[derive(Deserialize, Debug)]
pub struct ConnectedPeer {
    interface_id: String,
    public_key: String,
}

#[derive(Deserialize, Debug)]
pub struct TeamName {
    name: String,
}

/// Starts the booking process and either returns the credentials, in case of success, or the reason why the request couldn't be fulfilled
#[post("/reservation", format = "json", data = "<data>")]
async fn reserve(data: Json<BookingData>, db: Connection<Db>) -> String {
    start_booking(data.0, db).await
    }

/// Adds a new device to the Database
#[post("/add_device", format = "json", data = "<data>")]
async fn add_device(data: Json<DeviceData>, mut db: Connection<Db>) {
    let _ = add_device_to_db(data.0, &mut db).await;
}

/// Adds a new interface to the Database
#[post("/add_interface", format = "json", data = "<data>")]
async fn add_interface(data: Json<InterfaceData>, mut db: Connection<Db>) {
    let _ = add_interface_to_db(data.0, &mut db).await;
}

/// Cancels an ongoing booking and frees the interface for new booking requests
#[post("/end_reservation", format = "json", data = "<data>")]
async fn end_reservation(data: Json<TeamName>, db: Connection<Db>) {
    let _ = end_booking(data.0, db).await;
}

#[get("/")]
fn index() -> Template {
    Template::render("index", context! { field: "value" })
}


#[launch]
async fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .attach(Template::fairing())
        .mount("/", routes![index, reserve, add_device, add_interface, end_reservation])
        //.mount("/login", cred::routes())
}

