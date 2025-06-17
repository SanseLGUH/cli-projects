use reqwest::{Client, Error};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::path::PathBuf;
use failure::Error as RarError;

const NZGAMES: &str = "https://raw.githubusercontent.com/SanseLGUH/my-cli-scripts/refs/heads/main/nz_mini_games_installator/mini_games/metadata.json";
const RAWCONTENT: &str = "https://raw.githubusercontent.com/SanseLGUH/my-cli-scripts/refs/heads/main/nz_mini_games_installator/mini_games/AMONGUS";

#[derive(Debug, Serialize, Deserialize)]
pub struct MiniGames {
	path: String,
	description: String
}

pub type MiniGameCollection = HashMap<String, MiniGames>;

pub async fn games() -> Result<MiniGameCollection, Error> {
	let client = Client::new();

	match client.get(NZGAMES).send().await {
		Ok(resp) => {
			Ok( resp.json().await? )
		}
		Err(_) => {
			Ok( HashMap::new() )
 		}
	}
}

use std::io::Write;

pub async fn install(url: &str, output: &str) -> Result<(), Error> {
	let bytes = reqwest::get(url).await?.bytes().await?.to_vec();
	let mut f = std::fs::File::create(output).unwrap();
	f.write_all(&bytes).unwrap();

	Ok(())
} 

pub fn unpack(path: &str, output: &str) -> Result<(), RarError> {
	let archive = rar::Archive::extract_all( "assets/among.rar", output, "" )?;
	Ok(())
}
