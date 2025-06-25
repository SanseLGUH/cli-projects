use reqwest::{Client, Error as HttpError};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::path::PathBuf;

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

pub fn unpack(path: &str, output: &PathBuf) -> std::io::Result<()> {
	let file = std::fs::File::open(path)?;
	let mut archive = zip::ZipArchive::new(file)?;
	archive.extract(&output)?;
	
	Ok(())
}

pub fn backup(path: &PathBuf) -> std::io::Result<()> {
    let mods_path: PathBuf = path.join("mods");
    let backup_path: PathBuf = path.join("backup_nazzy_auto");

    if  mods_path.metadata()?.len() > 0 {
        if backup_path.exists() || backup_path.metadata()?.len() > 0 {  
            println!("\n[ {} ] Предыдущая резервная копия будет удалена. Для отмены нажмите Ctrl + C, или нажмите Enter для продолжения.", backup_path.display());
         
            let mut pause = String::new();
            std::io::stdin().read_line(&mut pause).unwrap();

            std::fs::remove_dir_all(&backup_path)?;
        }

        std::fs::rename(mods_path, backup_path)?;
    }    

    Ok(())
}
