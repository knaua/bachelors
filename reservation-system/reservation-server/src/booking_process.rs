use std::process::Command;
use crate::{BookingData, TeamName};
use crate::db_manager::{add_peer_to_db, change_availability, count_interfaces_from_db, remove_peer_from_interface, retrieve_connected_peer, retrieve_first_interface, Db};
use rocket_db_pools::Connection;


/// Book the in the request specified number of devices and return the credentials to access them if possible, or an error if the request is not processable
// TODO Return a proper error that a client may know the request didn't succeed
pub async fn start_booking(data: BookingData, mut db: Connection<Db>) -> String /*  Result<String, Err(T)> */ {

    // For the current capabilities of the system, this could be integrated into retrieve_interfaces, as the currently intended
    // functionality only supports one ESP32 per WireGuard interface. However, this will be kept separate in case of possibly adding
    // functionality for multiple devices in different interfaces later
    let available_interfaces = count_interfaces_from_db(&mut db).await;
    if !request_possible(data.devices, available_interfaces) {
        // TODO Better error handling
        return "No suitable interface available\nPlease try again later\n".to_string()
    }

    // Retrieve the first interface of the list of all available interfaces
    // This could use some additional mechanism to guarantee utilization of interfaces that end up lower on the list
    // inside the database simply because of their position in the table.

    let interface = retrieve_first_interface(&mut db).await;

    change_availability(&interface.interface_id, false, &mut db).await.expect("Couldn't change the availability of the interface");

    add_peer_to_db(&data.team_name, &interface.interface_id, &data.public_key, &mut db).await.expect("Couldn't create a database entry for the peer");

    let ip_address = interface.ip_address;

    // Constructs the IP-Address for the Peer. This assumes that all interfaces have an IP-Address in the form of "xx.0.0.x/24" where xx is in the range from 10-99,
    // use 24 as the subnet mask and that no device connected to the interface has the ip-address "xx.0.0.5/24"
    let slice =  &ip_address[0..2];
    let slice_as_value = slice.to_string();
    let peer_ip_address = [slice_as_value,".0.0.5".to_string()].join("");

    // starts the interface, but currently we only want to add peers to already running interfaces
    /*Command::new("sudo")
        .arg("wg-quick")
        .arg("up")
        .arg(&interface.interface_id.as_str()) // name of the interface to start
        .spawn()
        .expect("failed to execute process");*/

    // Add a peer to a running interface
    // This needs to be done with sudo due to WireGuard, for this work the 'wg' command needs to be added to sudoers
    Command::new("sudo")
        .arg("wg")
        .arg("set")
        .arg(&interface.interface_id.as_str())
        .arg("peer")
        .arg(&data.public_key.as_str())
        .arg("allowed-ips")
        .arg(peer_ip_address.clone()+"/32")
        .spawn()
        .expect("failed to add peer to interface");

    // peer_ip_address needs to be appended with the subnet mask of the chosen interface
    // for testing purposes all interfaces received the subnet mask 24 upon configuration
    let interface_credentials = [peer_ip_address+"/24", interface.host_public_key, interface.port].join("\n");
    interface_credentials

}

// TODO create a function that checks the WireGuard public key, if it matches its requirements
// If I can find out the specifics of its requirements, probably its length and the '=' at the end are good checkmarks?

/// Check that the requested number of devices does not exceed the number of available interfaces
fn request_possible (needed_devices: u8, available_interfaces: u8) -> bool{
    needed_devices <= available_interfaces
}

/// Ends a running reservation
pub async fn end_booking(team: TeamName, mut db: Connection<Db>) {
    let peer = retrieve_connected_peer(team.name, &mut db).await;

    // Remove the peer from the interface
    Command::new("sudo")
        .arg("wg")
        .arg("set")
        .arg(&peer.interface_id.as_str())
        .arg("peer")
        .arg(&peer.public_key.as_str())
        .arg("remove")
        .spawn()
        .expect("failed to remove peer from interface");

    // Make the interface available again
    change_availability(&peer.interface_id, true, &mut db).await.expect("Couldn't change the availability of the interface");

    // Remove the peer from the database
    remove_peer_from_interface(peer, &mut db).await.expect("Couldn't remove the peer from the database");
}

