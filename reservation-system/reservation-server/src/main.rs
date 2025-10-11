#[macro_use] extern crate rocket;

mod paste_ids;
mod booking_process;
mod db_manager;
//mod cred;

use paste_ids::PasteId;
use booking_process::book;
use rocket::response::content::RawText;
use rocket::tokio::fs::{self, File};
use rocket::serde::Deserialize;
use rocket::serde::json::Json;
use rocket_db_pools::{Connection, Database};
use tokio::task;
use crate::db_manager::{count_devices_from_db, write_into_db, Db};

const _ID_LENGTH: usize = 4;

/// Test value, to be used instead of the available devices noted in the database
pub const AVAILABLE_DEVICES: u8 = 5;
//const HOST: Absolute<'static> = uri!("http://localhost:8000");

//TODO Maybe change type of 'devices' and 'minutes' to u8 so parsing from string isn't necessary anymore
// -> currently keeping it as string is easier from the client side for testing
#[derive(Deserialize, Debug)]
pub struct BookingData {
    devices: String,
    minutes: String,
    team: String,
}

#[derive(Deserialize, Debug)]
pub struct DeviceData{
    mac_address: String,
    ip_address: String,
    available: String,
}

#[post("/reservation", format = "json", data = "<data>")]
async fn reserve(data: Json<BookingData>, mut db: Connection<Db>) -> std::io::Result<String>{

    //TODO adjust return type to something more accommodating to the response of a post request
    //TODO Saving information to a file might be helpful later
    //TODO Credentials could also be stored in a database like the (number of) devices
    /*let id = PasteId::new(ID_LENGTH);
    booking_info.open(128.kibibytes()).into_file(id.file_path()).await?;
    //Ok(uri!(HOST, retrieve(id)).to_string())
    Ok(id.to_string())*/

    /* Open the Data from the request and check it, then act accordingly to the available resources */
    // Number of available devices
    let x = count_devices_from_db(&mut db).await;
    println! ("{:?}", book(data.0, x));
    Ok("worked".to_string())
}

/// Adds a new device to the Database of available devices
#[post("/add_me", format = "json", data = "<data>")]
//TODO as this request should come from the ESPs, this probably needs to accept CBOR Data,
// however, the regular text could be utilized, although it probably needs to be able to decode what the ESP sent
// This also probably needs to be able to accept CoAP requests
//CoAP Functionality might prove to be problem, at least ESP32 seem powerful enough to handle HTTP, maybe JSON as well?
//Still, CoAP support would be nice, but it doesn't seem like rocket is compatible with that
//This might need another Host/Proxy/Gateway to relay and translate CoAP requests
async fn add_device(data: Json<DeviceData>, mut db: Connection<Db>) /*-> std::io::Result<String>*/ {
    let _ = write_into_db(data.0, &mut db).await;
}

#[post("/devices_available")] // not used currently, getting number of devices was relocated into the booking process post request handler
async fn _number_of_devices(mut db: Connection<Db>){
    count_devices_from_db(&mut db).await;
}

#[post("/test_change_available", format = "json", data = "<data>")]
async fn test_change_available(data: Json<DeviceData>, mut db: Connection<Db>) {}

#[get("/<id>")] // not currently in use, maybe if work with a file system is used in the future
async fn retrieve(id: PasteId<'_>) -> Option<RawText<File>> {
    File::open(id.file_path()).await.map(RawText).ok()
}

#[delete("/<id>")] // not currently in use, maybe if work with a file system is used in the future
async fn delete(id: PasteId<'_>) -> Option<()> {
    fs::remove_file(id.file_path()).await.ok()
}


#[get("/index")]
fn index() -> &'static str {
    "
    USAGE

      POST /

          accepts raw data in the body of the request and responds with a URL of
          a page containing the body's content

      GET /<id>

          retrieves the content for the paste with id `<id>`
    "
}

#[launch]
async fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .mount("/", routes![index, retrieve, reserve, delete, add_device])
}

