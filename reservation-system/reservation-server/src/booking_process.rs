use std::num::ParseIntError;
use crate::{BookingData, AVAILABLE_DEVICES};


/// Book the in the request specified number of devices and return the credentials to access them if possible, or an error if the request is not processable
// TODO Return the proper values, i. e. the credentials or a proper error
pub fn book(data: BookingData, devices_available: u8) -> String /*  Result<String, Err(T)> */ {

    /* Check if there are enough devices available, if not return error message */
    let x = parse_and_check(data.devices);

    if x.is_err() {
        // TODO Better error messages
       return "Error, not a number".to_string()
    }

    if !request_possible(x.unwrap(),devices_available) {
        // TODO Better error messages
        return "Not enough devices available".to_string()
    }

    /* Start the booking process:
     - Create access credentials for the devices.
     - Reserve the specified number of devices,
     - define them by their team name and
     - restrict access accordingly with the generated credentials */

    /* Return the access credentials for the reserved devices */

    let _unused_for_now = (data.minutes, data.team);
    /* TODO something with minutes and team */

    "credentials go here".to_string()
}

/// Parse the devices field from the JSON input to check if it matches a number
// Now also used to parse the value used to represent the State of a device in the database
pub fn parse_and_check(devices: String) -> Result<u8, ParseIntError> {
    let x = match devices.parse::<u8>(){
        Ok(x) => x,
        Err(e) => return Err(e),
    };
    Ok(x)
}

/// Check that the requested number of devices does not exceed the number of available devices
fn request_possible (x: u8, y: u8) -> bool{
    x <= y

    /* Update the number of available devices */
    // Here? or in another function?
}

#[deprecated]
/// Get the number of available devices located in the network
async fn _number_of_devices() -> u8 {
    // dummy value for testing
    AVAILABLE_DEVICES
}