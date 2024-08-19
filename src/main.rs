use reqwest::blocking::Client;
use std::io;
use serde_json::json;
use serde::Deserialize;
use regex::Regex;

#[derive(Deserialize, Debug)]
struct ResponseItem {
    id: String,
    name: String,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // get username from user input
    let mut input = String::new();
    println!("Username:");
    io::stdin().read_line(&mut input)?;
    let input = input.trim(); // remove trailing newline

    // make a json request body
    let body = json!([input]);

    // create a HTTP client and send a POST request
    let client = Client::new();

    // Send POST request to LittleSkin
    let res = client.post("https://littleskin.cn/api/yggdrasil/api/profiles/minecraft")
        .json(&body)
        .send()?;

    // Get response from LittleSkin
    let response_text = res.text()?;
    println!("Response From LittleSkin Yggdrasil API: {:?}\n", response_text);

    // Parse JSON response from LittleSkin
    let items: Vec<ResponseItem> = serde_json::from_str(&response_text)?;

    // Use regex to format UUID
    let re = Regex::new(r"([a-fA-F0-9]{8})([a-fA-F0-9]{4})([a-fA-F0-9]{4})([a-fA-F0-9]{4})([a-fA-F0-9]{12})").unwrap();

    for item in items {
        if let Some(caps) = re.captures(&item.id) {
            let formatted_uuid = format!(
                "{}-{}-{}-{}-{}",
                &caps[1], &caps[2], &caps[3], &caps[4], &caps[5]
            );
            println!("Parsed Response from LittleSkin");
            println!("Username: {}", item.name);
            println!("UUID: {}", formatted_uuid);
        } else {
            println!("Something wrong. Received: {}", item.id);
        }
    }

    println!("Wait, trying to get UUID from Mojang API...\n");

    // Send POST request to Mojang
    let res = client.post("https://authserver.mojang.com/api/profiles/minecraft")
        .json(&body)
        .send()?;

    // Get response from Mojang
    let response_text = res.text()?;
    println!("Response From Mojang API: {:?}\n", response_text);

    // Parse JSON response from Mojang
    let items: Vec<ResponseItem> = serde_json::from_str(&response_text)?;

    for item in items {
        if let Some(caps) = re.captures(&item.id) {
            let formatted_uuid = format!(
                "{}-{}-{}-{}-{}",
                &caps[1], &caps[2], &caps[3], &caps[4], &caps[5]
            );
            println!("Parsed Response from Mojang");
            println!("Username: {}", item.name);
            println!("UUID: {}", formatted_uuid);
        } else {
            println!("Something wrong. Received: {}", item.id);
        }
    }

    Ok(())
}
