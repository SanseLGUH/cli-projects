
use reqwest::{header::{HeaderMap, HeaderValue}, Client};
use serde::{Deserialize, Serialize};
use std::io::{self, Write};

#[derive(Serialize, Deserialize, Debug)]
struct OpenedChannels {
    id: String,
}

#[tokio::main]
async fn main() {
    print!("Type your token: ");
    io::stdout().flush().unwrap();

    let mut token = String::new();
    io::stdin().read_line(&mut token).unwrap();
    let token = token.trim(); 

    let mut headers = HeaderMap::new();
    headers.insert(
        "Authorization",
        HeaderValue::from_str(token).expect("Invalid token format"),
    );

    let client = Client::builder()
        .default_headers(headers)
        .build()
        .expect("Failed to build client");

    let opened_channels: Vec<OpenedChannels> = client
        .get("https://discord.com/api/v9/users/@me/channels")
        .send()
        .await
        .expect("Failed to fetch channels")
        .json()
        .await
        .expect("Failed to parse channel JSON");

    for channel in &opened_channels {
        let res = client
            .delete(&format!(
                "https://discord.com/api/v9/channels/{}?silent=true",
                channel.id
            ))
            .send()
            .await;

        match res {
            Ok(response) => println!("Deleted channel {}: {}", channel.id, response.status()),
            Err(e) => eprintln!("Failed to delete channel {}: {:?}", channel.id, e),
        }
    }

    println!("Done");
}

