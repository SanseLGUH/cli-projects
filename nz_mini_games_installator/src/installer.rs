use reqwest::{Client, Error as HttpError};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

const NZGAMES: &str = "https://raw.githubusercontent.com/SanseLGUH/my-cli-scripts/refs/heads/main/nz_mini_games_installator/mini_games/metadata.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct MiniGames {
	pub path: String,
	pub description: String
}

pub type MiniGameCollection = HashMap<String, MiniGames>;

pub async fn games() -> Result<MiniGameCollection, HttpError> {
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

pub async fn install(url: &str, output: &str) -> Result<(), HttpError> {
	let bytes = reqwest::get(url).await?.bytes().await?.to_vec();
	let mut f = std::fs::File::create(output).unwrap();
	f.write_all(&bytes).unwrap();

	Ok(())
} 

pub fn unpack(path: &str, output: &str) -> std::io::Result<()> {
	let file = std::fs::File::open(path)?;
	let mut archive = zip::ZipArchive::new(file)?;
	archive.extract(&std::path::Path::new(output))?;
	
	Ok(())
}
