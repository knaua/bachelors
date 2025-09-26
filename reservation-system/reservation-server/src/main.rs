#[macro_use] extern crate rocket;

mod paste_ids;
mod booking_process;

use paste_ids::PasteId;
use booking_process::book;
use rocket::response::content::RawText;
use rocket::tokio::fs::{self, File};
use rocket::serde::Deserialize;
use rocket::serde::json::Json;
use rocket_db_pools::{sqlx, Database};

const _ID_LENGTH: usize = 4;
pub const AVAILABLE_DEVICES: u8 = 5;
//const HOST: Absolute<'static> = uri!("http://localhost:8000");

#[derive(Database)]
#[database("main")]
struct Db(sqlx::SqlitePool);

// TODO Maybe change type of 'devices' and 'minutes' to u8 so parsing from string isn't necessary anymore -> currently keeping it as string is easier from the client side for testing
#[derive(Deserialize, Debug)]
pub struct BookingData {
    devices: String,
    minutes: String,
    team: String,
}

#[post("/reservation", format = "json", data = "<booking_info>")]
async fn reserve(booking_info: Json<BookingData>) -> std::io::Result<String>{


    //TODO Saving information to a file might be helpful later
    //TODO Credentials could also be stored in a database like the (number of) devices
    /*let id = PasteId::new(ID_LENGTH);
    booking_info.open(128.kibibytes()).into_file(id.file_path()).await?;
    //Ok(uri!(HOST, retrieve(id)).to_string())
    Ok(id.to_string())*/

    /* Open the Data from the request and check it, then act accordingly to the available resources */
    println! ("{:?}", book(booking_info.0));
    Ok("worked".to_string())
}

#[get("/<id>")] //TODO: Authentication Data Check to retrieve the Data
async fn retrieve(id: PasteId<'_>) -> Option<RawText<File>> {
    File::open(id.file_path()).await.map(RawText).ok()
}

#[delete("/<id>")]
async fn delete(id: PasteId<'_>) -> Option<()> {
    fs::remove_file(id.file_path()).await.ok()
}


#[get("/")]
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
fn rocket() -> _ {
    rocket::build()
        .attach(Db::init())
        .mount("/", routes![index, retrieve, reserve, delete])
}

