//use std::process::Command;
use crate::{InterfaceData, PeerPubKey};
use crate::db_manager::{add_peer_to_db, change_availability, count_interfaces_from_db, retrieve_first_interface, Db};
use rocket_db_pools::Connection;

// Static Value used in testing, assuming each client may book only one device at a time. Adjust accordingly to own needs.
// Functionality for a variable number of devices not yet implemented.
// It would be best to change to JSON Datatype passed to the 'book' function to contain a specified number of devices,
// which can be set in the booking request sent to the server (e.g. see the struct BookingData in main.rs)
static NEEDED_DEVICES: u8 = 1;
static _BOOKING_DURATION: u8 = 120;

/// Book the in the request specified number of devices and return the credentials to access them if possible, or an error if the request is not processable
// TODO Return a proper error that a client may know the request didn't succeed
pub async fn book(peer: PeerPubKey, mut db: Connection<Db>) -> String /*  Result<String, Err(T)> */ {

    /* Check if there are enough devices available, if not return error message */

    // For the current capabilities of the system, this could be integrated into retrieve_interfaces, as the currently intended
    // functionality only supports one ESP32 per WireGuard interface. However, this will be kept separate in case of possibly adding
    // functionality for multiple devices in different interfaces later
    let available_interfaces = count_interfaces_from_db(&mut db).await;
    if !request_possible(NEEDED_DEVICES, available_interfaces) {
        // TODO Better error handling
        return "No suitable interface available\n".to_string()
    }

    // Retrieve the first interface of the list of all available interfaces
    // This could use some additional mechanism to guarantee utilization of interfaces that end up lower on the list
    // simply because of their position in the database. Maybe add some counter to the interfaces inside the database
    // that can be used to sort them by least recently used?

    let interface = retrieve_first_interface(&mut db).await;

    //change_availability(&interface.interface_id, false, &mut db).await.expect("TODO: panic message");
    println!("availability was changed");

    //add_peer_to_db(&interface.interface_id, peer.public_key, &mut db).await.expect("TODO: panic message");
    println!("peer was added to the db");

    // TODO Get the interfaces IP-Address to create a new IP-Address for the peer from it

    let ip_address = interface.ip_address;

    // Constructs the IP-Address for the Peer. This assumes that all interfaces have an IP-Address in the form of "xx.0.0.x/24" where xx is in the range from 10-99,
    // use 24 as the subnet mask and that no device connected to the interface has the ip-address "xx.0.0.5/24"
    let slice =  &ip_address[0..2];
    let mut slice_as_value = slice.to_string();
    let peer_ip_address = [slice_as_value,".0.0.5".to_string()].join("");

    // starts the interface, but currently we only want to add peers to already running interfaces
    // wg-quick requires the root password when called
    /*Command::new("wg-quick")
        .arg("up")
        .arg(&interface.interface_id.as_str()) // name of the interface to start
        .spawn()
        .expect("failed to execute process");*/

    // add a peer to a running interface
    // operation is not permitted due to lack of permissions
    // TODO Find a method to add a new peer to an existing interface automatically without the need for user input
    /*Command::new("wg")
        .arg("set")
        .arg(&interface.interface_id.as_str())
        .arg("peer")
        .arg(&peer.public_key.as_str())
        .arg("allowed-ips")
        .arg(peer_ip_address.clone()+"/32")
        .spawn()
        .expect("failed to execute process");*/

    // peer_ip_address needs to be appended with the subnet mask of the chosen interface
    // for testing purposes all interfaces received the subnet mask 24 upon configuration
    let interface_credentials = [peer_ip_address+"/24", interface.host_public_key, interface.port].join("\n");
    interface_credentials

}

// TODO create a function that checks the wireguard public key, if it matches its requirements
// If I can find out the specifics of its requirements, probably its length and the '=' at the end are good checkmarks?

/// Check that the requested number of devices does not exceed the number of available interfaces
fn request_possible (needed_devices: u8, available_interfaces: u8) -> bool{
    needed_devices <= available_interfaces
}
