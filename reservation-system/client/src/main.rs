extern crate coap;

use std::collections::HashMap;
use reqwest::Client;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let url = "http://127.0.0.1:8000/reservation";
    //println!("Client request: {}", url);
    let _id = "http://127.0.0.1:8000/";

    let _booking_data = [("devices", "3"), ("minutes", "120"), ("team_name", "makoo")];

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
    let body = get([id,"MIT2"].join(""))
        .await?
        .text()
        .await?;
    //let info = body.split('=').collect::<Vec<&str>>();
    println!("body: {}", body);
    //println!("body: {:?}", info);
    */


    // Post as JSON   // ID from first use: eXNH

    let mut map = HashMap::new();
    map.insert("devices", "7");
    map.insert("minutes", "90");
    map.insert("team","okamo");


    let client = Client::new();
    /*let res = client.post(url)
        .json(&map)
        .send()
        .await?;*/

    //println!("{:?}", map);

    //println!("body: {}", res.text().await?);

    let res = client.post("http://127.0.0.1:8000/devices_available")
        .send().await?;

    Ok(())
}
