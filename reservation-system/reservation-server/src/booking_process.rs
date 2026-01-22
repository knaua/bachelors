use std::process::Command;
use ipnet::Ipv4Net;
use crate::{BookingData, TeamName};
use crate::db_manager::{add_peer_to_db, change_availability, count_interfaces_from_db, remove_peer_from_interface, retrieve_connected_peer, retrieve_first_interface, Db};
use rocket_db_pools::Connection;

static DEVICES: u8 = 1;

/// Book the in the request specified number of devices and return the credentials to access them if possible, or an error if the request is not processable
// TODO Return a proper error that a client may know the request didn't succeed
// TODO Check available IP-Addresses for an interface and assign them dynamically
pub async fn start_booking(data: BookingData, mut db: Connection<Db>) -> String {

    // For the current capabilities of the system, this could be integrated into retrieve_interfaces, as the currently intended
    // functionality only supports one ESP32 per WireGuard interface. However, this will be kept separate in case of possibly adding
    // functionality for multiple devices in different interfaces later
    let available_interfaces = count_interfaces_from_db(&mut db).await;
    if !request_possible(DEVICES, available_interfaces) {
        return "\nNo_suitable_interface_available".to_string()
    }

    // Retrieve the first interface of the list of all available interfaces
    // This could use some additional mechanism to guarantee utilization of interfaces that end up lower on the list
    // inside the database simply because of their position in the table.
    let interface = retrieve_first_interface(&mut db).await;

    // Add the peer to the database, this currently panics if the team name is already present in the database
    add_peer_to_db(&data.team_name, &interface.interface_id, &data.public_key, &mut db).await.expect("Couldn't create a database entry for the peer");

    // Changes the availability of the interface to 0 in the database
    change_availability(&interface.interface_id, false, &mut db).await.expect("Couldn't change the availability of the interface");

    // Retrieve the interfaces IP-Address and its prefix
    let ip_address: Ipv4Net = interface.ip_address.parse().unwrap();
    let prefix = ip_address.prefix_len().to_string();

    // Construct a new IP-Address for the peer to be added
    // As there currently is no mechanism to check the IP-Addresses already in use in an interface, xxx.xxx.xxx.5 is used for teams
    let server_ip_address = &ip_address.addr().to_string();
    let mut peer_ip_address = server_ip_address.split('.').collect::<Vec<&str>>();
    peer_ip_address.remove(peer_ip_address.len() - 1);
    peer_ip_address.push("5");
    let peer_ip_address = peer_ip_address.join(".");

    // Add a peer to a running interface
    wireguard_add_peer(&interface.interface_id, &data.public_key, peer_ip_address.clone());

    let interface_credentials = [peer_ip_address+"/"+&prefix, interface.host_public_key, interface.port].join("\n");
    interface_credentials
}

/// Check that at least one interface is available
// This function should later check the number of devices connected to an available interface to properly fulfill a request by the number of needed devices instead
fn request_possible (needed_devices: u8, available_interfaces: u8) -> bool{
    available_interfaces >= needed_devices
}

/// Ends a running reservation
pub async fn end_booking(team: TeamName, mut db: Connection<Db>) {
    let peer = retrieve_connected_peer(team.team, &mut db).await;

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

/// Connect a peer to an existing and running interface
/// For this to be usable change the sudoers file to allow the execution of 'wg' without a password
pub fn wireguard_add_peer(interface: &str, peer_pub_key: &str, peer_ip_address: String) {
    Command::new("sudo")
        .arg("wg")
        .arg("set")
        .arg(interface)
        .arg("peer")
        .arg(peer_pub_key)
        .arg("allowed-ips")
        .arg(peer_ip_address+"/32")
        .spawn()
        .expect("failed to add peer to interface");
}

