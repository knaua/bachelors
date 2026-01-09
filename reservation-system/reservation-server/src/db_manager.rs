use rocket_db_pools::{sqlx, Database};
use rocket_db_pools::sqlx::{query, Connection, Error, Row, SqliteConnection, SqlitePool};
use rocket_db_pools::sqlx::sqlite::SqliteRow;
use crate::{DeviceData, InterfaceData, PeerPubKey};


#[derive(Database, Debug)]
#[database("main")]
pub struct Db(SqlitePool);

// TODO Restrict access to the Database, no one should be able to make calls to the database without permission
// currently anyone could just add, remove or change entries if the URIs are known...


/// Returns the number of available devices from the database
pub async fn count_interfaces_from_db(conn: &mut SqliteConnection) -> u8{ // TODO change return type and catch possible unwrap error
    let result = query("SELECT * FROM interfaces WHERE available=1").fetch_all(&mut *conn).await.unwrap();
    result.iter().count() as u8
}

/// Adds a new device to the database
pub async fn add_device_to_db(data: DeviceData, conn: &mut SqliteConnection) -> Result<(), sqlx::Error> {
    //TODO Check for double entries and catch resulting errors appropriately
    let _result = query("INSERT INTO devices (mac_address, ip_address, interface_port, interface_id)\
    VALUES (?1, ?2, ?3, ?4)")
        .bind(data.mac_address)
        .bind(data.ip_address)
        .bind(data.interface_port)
        .bind(data.interface_id)
        .execute(conn)
        .await;
    Ok(())
}

/// Adds a new interface to the database
pub async fn add_interface_to_db(data: InterfaceData, conn: &mut SqliteConnection) -> Result<(), sqlx::Error> {
    //TODO Check for double entries and catch resulting errors appropriately
    let _result = query("INSERT INTO interfaces (interface_id, ip_address, port, host_public_key, available)\
    VALUES (?1, ?2, ?3, ?4, ?5)")
        .bind(data.interface_id)
        .bind(data.ip_address)
        .bind(data.port)
        .bind(data.host_public_key)
        .bind(data.available)
        .execute(conn)
        .await;
    Ok(())
}

/// Adds a new peer by its public key and the interface it is connected to
pub async fn add_peer_to_db(id: &String, key: &String, conn: &mut SqliteConnection) -> Result<(), sqlx::Error> {
    let _result = query("INSERT INTO peers (interface_id, peer_public_key)\
    VALUES (?1, ?2)")
        .bind(&id)
        .bind(&key)
        .execute(conn)
        .await;
    Ok(())
}

/// Removes a peer from the interface it is connected to
pub async fn remove_peer_from_interface(peer: PeerPubKey, conn: &mut SqliteConnection) -> Result<(), sqlx::Error> {
    let _result = query("DELETE FROM peers WHERE peer_public_key = ?1")
        .bind(peer.public_key)
        .execute(conn)
        .await;
    Ok(())
}

/// Change the availability of an interface
pub async fn change_availability(id: &String, available: bool, conn: &mut SqliteConnection) -> Result<(), sqlx::Error> {
    let x = match available {
        true => 1,
        false => 0,
    };
    
    let _result = query("UPDATE interfaces SET available=?1 WHERE id=?2")
        .bind(x)
        .bind(&id)
        .execute(conn)
        .await;
    Ok(())
}

/// Retrieves a vector of all available interfaces
pub async fn _retrieve_interfaces(conn: &mut SqliteConnection) -> Vec<InterfaceData>{
    let result = query("SELECT * FROM interfaces WHERE available=1").fetch_all(&mut *conn).await.unwrap();
    let mut interface_vector: Vec<InterfaceData> = vec![];

    for i in 0..result.len() {
        let x = InterfaceData {
            interface_id: result.get(i).unwrap().get(0),
            ip_address: result.get(i).unwrap().get(1),
            port: result.get(i).unwrap().get(2),
            host_public_key: result.get(i).unwrap().get(3),
            available: result.get(i).unwrap().get(4) };
        interface_vector.push(x);
    };
     interface_vector
}

/// Retrieves the first available interface
pub async fn retrieve_first_interface(conn: &mut SqliteConnection) -> InterfaceData{
    let result = query("SELECT * FROM interfaces WHERE available=1").fetch_all(&mut *conn).await.unwrap();
    let interface = InterfaceData {
        interface_id: result.get(0).unwrap().get(0),
        ip_address: result.get(0).unwrap().get(1),
        port: result.get(0).unwrap().get(2),
        host_public_key: result.get(0).unwrap().get(3),
        available: result.get(0).unwrap().get(4)
    };
    interface
}

/// Check if the provided user and password match the user's id and his password
pub async fn _get_login(user: &str, pw: &str) -> Result<(String, String), Error>{ // TODO ENCRYPT/HASH PASSWORDS!!!
    let mut co: SqliteConnection = SqliteConnection::connect("main.sqlite").await?;
    let result = query("SELECT * FROM users WHERE id = ?").bind(user.to_string()).fetch_one(&mut co).await?;
    let credentials: (String, String) = (result.get("id"), result.get("password"));
    if  credentials.1 == pw && credentials.0 == user{
        Ok(credentials)
    } else {
        Err(Error::RowNotFound)
    }

}

/// Check if the provided user id exists in the database
pub async fn _get_user(user: &str) -> Result<SqliteRow, Error>{ // TODO ENCRYPT/HASH PASSWORDS!!!
    let mut co: SqliteConnection = SqliteConnection::connect("main.sqlite").await?;
    let result = query("SELECT * FROM users WHERE id = ?").bind(user.to_string()).fetch_one(&mut co).await?;
    let id: String = result.get("id");
    if  id == user{
        Ok(result)
    } else {
        Err(Error::RowNotFound)
    }

}
