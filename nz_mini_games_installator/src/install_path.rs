use std::{fs::*, path::PathBuf};
use dirs::config_dir;

pub fn default_minecraft_path() -> Option<PathBuf> {
	let default_path = config_dir().unwrap().join(".minecraft");

    match default_path.exists() {
    	true => Some( default_path ),
    	false => None
    }
}

pub fn clear_mods(path: &PathBuf) -> std::io::Result<()> {
    remove_dir_all(path.join("mods"))
}
