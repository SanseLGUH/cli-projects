use reqwest::{Client, Error};
use serde::Serialize;
use std::path::PathBuf;

const NZGAMES: &str = "https://github.com/SanseLGUH/my-cli-scripts/tree-commit-info/main/nz_mini_games_installator/mini_games";
const RAWCONTENT: &str = "https://raw.githubusercontent.com/SanseLGUH/my-cli-scripts/refs/heads/main/nz_mini_games_installator/mini_games/";

pub async fn games() -> Vec<String> {
	let client = Client::new();

	if let Ok(resp) = client.get(NZGAMES).send().await {
		println!("{:?}", resp);
	}

	vec![ String::new() ]
}

struct Installation {
	path: PathBuf
}

impl Installation {
	pub async fn install() -> Result<Self, Error> {
		todo!()
	} 

	pub fn unpack() {
	}
}
