#[macro_use] extern crate rocket;

mod booking_process;
mod db_manager;
mod cred;


use booking_process::book;
use rocket::serde::Deserialize;
use rocket::serde::json::Json;
use rocket_db_pools::{Connection, Database};
use rocket_dyn_templates::{Template, context};

use crate::db_manager::{add_interface_to_db, add_device_to_db, Db};
// Remove later, these functions are only callable from here for testing
use crate::db_manager::{change_availability, retrieve_first_interface};


#[derive(Deserialize, Debug)]
pub struct _BookingData {
    _devices: i32,
    _minutes: i32,
    _team: String,
}

// Does this all make sense for the devices (ESP32), they should already be initialized with the interfaces port it is connected to
#[derive(Deserialize, Debug)]
pub struct DeviceData{
    mac_address: String,
    ip_address: String, // this may also be irrelevant, I'm unsure about that however
    interface_port: String, // we probably don't need this for the ESP32
    interface_id: String,
}

#[derive(Deserialize, Debug)]
pub struct InterfaceData{
    interface_id: String,
    ip_address: String,
    port: String,
    host_public_key: String,
    available: u8,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct PeerPubKey {
    public_key: String,
}

/// Starts the booking process and either returns the credentials, in case of success, or the reason why the request couldn't be fulfilled
#[post("/reservation", format = "json", data = "<data>")]
async fn reserve(data: Json<PeerPubKey>, db: Connection<Db>) -> String {
    book(data.0, db).await
    }

/// Adds a new device to the Database
#[post("/add_device", format = "json", data = "<data>")]
async fn add_device(data: Json<DeviceData>, mut db: Connection<Db>) /*-> std::io::Result<String>*/ {
    let _ = add_device_to_db(data.0, &mut db).await;
}

/// Adds a new interface to the Database
#[post("/add_interface", format = "json", data = "<data>")]
async fn add_interface(data: Json<InterfaceData>, mut db: Connection<Db>) /*-> std::io::Result<String>*/ {
    let _ = add_interface_to_db(data.0, &mut db).await;
}

// Only for testing
#[post("/avai")]
async fn avai(mut db: Connection<Db>) {
    let _ = change_availability(&"intf1".to_string(), true, &mut db).await;
}

// Only for testing
#[post("/unavai")]
async fn unavai(mut db: Connection<Db>) {
    let _ = change_availability(&"intf1".to_string(), false, &mut db).await;
}

// Only for testing
#[post("/getall")]
async fn getall(mut db: Connection<Db>) {
    let _ = retrieve_first_interface(&mut db).await;
}

#[get("/home")]
fn home() -> &'static str {
    "
    USAGE

      POST /

          accepts raw data in the body of the request and responds with a URL of
          a page containing the body's content

      GET /<id>

          retrieves the content for the paste with id `<id>`
    "
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
        .mount("/", routes![index, home, reserve, add_device, add_interface, avai, unavai, getall])
        .mount("/login", cred::routes())
}

