use std::collections::HashMap;
use reqwest::{Client, get};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let url = "http://127.0.0.1:8000/reservation";
    let add_device = "http://127.0.0.1:8000/add_me";
    let _id = "http://127.0.0.1:8000/";
    let _booking_data = [("devices", "3"), ("minutes", "120"), ("team_name", "makoo")];
    let index = "http://192.168.178.55:8000/index";

    // Post as form
    /*
    let client = Client::new();
    let request = client.post(url)
        //.form(&booking_data)
        .body("3,60,ookma")
        .send()
        .await.unwrap();
    println!("{:?}", request.text().await); // ID needs to be stored upon reception or at least the user needs to have access to it, in order for them to access the reserved devices
    */

    // Get Data of certain ID (Data created with post over body)

    /*
    let body = get(index)
        .await?
        .text()
        .await?;
    //let info = body.split('=').collect::<Vec<&str>>();
    println!("body: {}", body);
    //println!("body: {:?}", info); */

    let body = get("http://127.0.0.1:8000/usr")
        .await?
        .text()
        .await?;
    //let info = body.split('=').collect::<Vec<&str>>();
    println!("user: {}", body);



    // Post as JSON   // ID from first use: eXNH

    let mut map = HashMap::new();
    map.insert("devices", "1");
    map.insert("minutes", "90");
    map.insert("team","okamo");

    let mut new_device = HashMap::new();
    new_device.insert("mac_address", "test-mac");
    new_device.insert("ip_address", "ip-test");
    new_device.insert("available", "1");


    println!("{:?}", map);

    /*
    let client = Client::new();
    let res = /*client.post(add_device)*/ client.post(url)
        .json(&map) // map for the booking request
        //.json(&new_device) // map for adding a new device to the database
        .send()
        .await?; */

    Ok(())
}
