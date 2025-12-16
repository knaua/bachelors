use std::num::ParseIntError;
//use std::process::Command;
use crate::PeerPubKey;
use crate::db_manager::{count_interfaces_from_db, add_device_to_db, Db};
use rocket_db_pools::Connection;

// Static Value used in testing, assuming each client may book only one device at a time. Adjust accordingly to own needs.
// Functionality for a variable number of devices not yet implemented.
// It would be best to change to JSON Datatype passed to the 'book' function to contain a specified number of devices,
// which can be set in the booking request sent to the server (e.g. see the struct BookingData in main.rs)
static NEEDED_DEVICES: u8 = 1;
static _BOOKING_DURATION: u8 = 120;

/// Book the in the request specified number of devices and return the credentials to access them if possible, or an error if the request is not processable
// TODO Return the proper values, i. e. the credentials or a proper error
pub async fn book(key: PeerPubKey, mut db: Connection<Db>) -> String /*  Result<String, Err(T)> */ {

    /* Check if there are enough devices available, if not return error message */
    //let devices_requested = parse_and_check(data.devices);


    if !check_key(key) {
        // TODO Better error messages
       return "Wrong public key format\n".to_string()
    }

    let available_interfaces = count_interfaces_from_db(&mut db).await;

    if !request_possible(NEEDED_DEVICES, available_interfaces) {
        // TODO Better error messages
        // TODO Maybe change to checking available WireGuard Interfaces instead of the devices itself, depends on later implementation of static or dynamic interfaces
        return "Not enough devices available\n".to_string()
    }

    /* Start the booking process:
     - Reserve the specified number of devices,
     - choose from the available devices and save the corresponding interface's IP-Address of the server
     - set up the WireGuard interface
     - send the server's public key, IP-Address and
     - the peer's designated IP-Address*/


    /* // needs root access if invoked, can this be supplied automatically or directly called as root?
    Command::new("wg-quick")
        .arg("up")
        .arg("wg") // name of the interface to start
        .spawn()
        .expect("failed to execute process");
     */

  "credentials go here\n".to_string()
}

/// Parse the devices field from the JSON input to check if it matches a number
// Now also used to parse the value used to represent the State of a device in the database
pub fn parse_and_check_u8(devices: String) -> Result<u8, ParseIntError> {
    let x = match devices.parse::<u8>(){
        Ok(x) => x,
        Err(e) => return Err(e),
    };
    Ok(x)
}

// TODO create a function that checks the wireguard public key, if it matches its requirements
// If I can find out the specifics of its requirements, probably its length and the '=' at the end are good checkmarks?

/// Check that the requested number of devices does not exceed the number of available interfaces
// TODO probably put this into the db_manager crate as the we directly interact with the database to get the needed information
fn request_possible (needed_devices: u8, available_interfaces: u8) -> bool{
    needed_devices <= available_interfaces

    /* Update the number of available devices */
    // Here? or in another function?
}
fn check_key(key: PeerPubKey) -> bool {
    /* Check if the key matches the format of a WireGuard Key */
    // TODO
    true
}