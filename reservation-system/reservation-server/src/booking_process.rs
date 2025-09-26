use std::num::ParseIntError;
use crate::{BookingData, AVAILABLE_DEVICES};

/// Book the in the request specified number of devices and return the credentials to access them if possible, or an error if the request is not processable
// TODO Return the proper values, i. e. the credentials or an error
pub fn book(data: BookingData) -> String /*  Result<String, Err(T)> */ {

    /* Check if there are enough devices available, if not return error message */
    let x = parse_and_check(data.devices);

    if x.is_err() {
        // TODO Better error messages
       return "Error, not a number".to_string()
    }

    if !request_possible(x.unwrap()) {
        // TODO Better error messages
        return "Not enough devices available".to_string()
    }

    /* Start the booking process:
     - Create access credentials for the devices.
     - Reserve the specified number of devices,
     - define them by their team name and
     - restrict access accordingly with the generated credentials */

    /* Return the access credentials for the reserved devices */

    let test = "credentials go here".to_string();
    test
}

/// Parse the devices field from the JSON input to check if it matches a number
fn parse_and_check(devices: String) -> Result<u8, ParseIntError> {
    let x = match devices.parse::<u8>(){
        Ok(x) => x,
        Err(e) => return Err(e),
    };
    Ok(x)
}

/// Check that the requested number of devices does not exceed the number of available devices
fn request_possible (x: u8) -> bool{
    let y = number_of_devices();
    x <= y

    /* Update the number of available devices */
    // Here? or in another function?
}

/// Get the number of available devices located in the network
//TODO Return type may change later
fn number_of_devices() -> u8 {

    /* How to determine number of available devices?
        Idea 1: Use a counter that can be accessed and manipulated with HTTP requests
        Idea 2: Use a database which keeps track of the available devices */

    // dummy value for testing
    AVAILABLE_DEVICES
}