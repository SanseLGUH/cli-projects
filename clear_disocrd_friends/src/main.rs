use reqwest::Client;
use std::env;

#[derive(serde::Deserialize, Debug)]
struct Friend {
    id: String
}

async fn get_friend_vec( client: &Client, token: &str ) -> Result<Vec<Friend>, reqwest::Error> {
    Ok( client
        .get("https://discord.com/api/v9/users/@me/relationships")
        .header("Authorization", token)
        .send().await?
        .json().await? )
}

async fn delete_friend( client: &Client, id: String, token: &str ) -> Result< (), reqwest::Error > {
    client.delete( format!("https://discord.com/api/v9/users/@me/relationships/{id}") )
        .header("Authorization", token)
        .send().await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!(" -token < discord-token > ");
        return;
    }
    
    let client = Client::new();
    
    let friend_list = get_friend_vec(&client, &args[2]).await.unwrap();

    for friend in friend_list {
        println!("{:?}", friend);

        delete_friend( &client, friend.id, &args[2] ).await.unwrap();
    }

    println!("Done");
}
