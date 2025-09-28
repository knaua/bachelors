use rocket_db_pools::Database;
use rocket_db_pools::sqlx::{query, SqliteConnection, SqlitePool};

#[derive(Database, Debug)]
#[database("main")]
pub struct Db(SqlitePool);

/// Returns the number of available devices from the database
pub async fn count_devices_from_db(conn: &mut SqliteConnection) -> u8{ // TODO change return type and catch possible unwrap error
    let qr = query("SELECT * FROM devices WHERE available=1").fetch_all(&mut *conn).await.unwrap();
    //println!("{:?}", qr.iter().count()); // count the actual number of rows "with available devices"
    qr.iter().count() as u8
}

/// Adds a new device (or even credentials) to the database
fn _write() {

}