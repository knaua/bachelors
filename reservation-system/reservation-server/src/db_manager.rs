use rocket_db_pools::{sqlx, Database};
use rocket_db_pools::sqlx::{query, SqliteConnection, SqlitePool};
use rocket_db_pools::sqlx::sqlite::SqliteQueryResult;
use rocket::http::{Cookie, CookieJar};
use crate::DeviceData;
use crate::booking_process::parse_and_check; // Doesn't feel right to do it this way, maybe this needs to be in another crate apart from the booking process,
                                             // something called 'utility' maybe?

#[derive(Database, Debug)]
#[database("main")]
pub struct Db(SqlitePool);

//TODO Restrict access to the Database, no one should be able to make calls to the database without permission
// currently anyone could just add, remove or change entries if the URIs are known...
// Then the devices would need to acquire credentials

/// Returns the number of available devices from the database
pub async fn count_devices_from_db(conn: &mut SqliteConnection) -> u8{ // TODO change return type and catch possible unwrap error
    let result = query("SELECT * FROM devices WHERE available=1").fetch_all(&mut *conn).await.unwrap();
    result.iter().count() as u8
}

/// Adds a new device (or even credentials) to the database
pub async fn write_into_db(data: DeviceData, conn: &mut SqliteConnection) -> Result<(), sqlx::Error> {
    /* Take incoming data and write it into the database. Check for duplicate MAC-/IP-Addresses and/or catch the resulting error */
    //TODO Make sure that the supplied MAC- and IP-Addresses really are those and not some random String
    //TODO Check for double entries and catch resulting errors appropriately
    //TODO Add 'create table if not exists'?
    let x = parse_and_check(data.available);
    let _result = query("INSERT INTO test (mac_address, ip_address, available)\
    VALUES (?1, ?2, ?3)") //TODO change 'test' back to 'devices' once done testing
        .bind(data.mac_address)
        .bind(data.ip_address)
        .bind(x.unwrap())
        .execute(conn)
        .await;
    Ok(())
}

async fn make_available(data: DeviceData, conn: &mut SqliteConnection) -> Result<(), sqlx::Error> {
    //TODO make new parameter to call the function, or make different functions for changing 'available' to either 1 or 0? Change function name then accordingly
    // Do I need the devices IP-Address for anything in particular? I probably need a function to change the IP-Address as well?
    let _result = query("UPDATE test\
    SET mac_address = ?1 \
    WHERE available=1")
        .bind(data.mac_address)
        .execute(conn)
        .await;
    Ok(())
}