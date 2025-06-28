use std::{fs::*, path::PathBuf};
use dirs::config_dir;

pub fn default_minecraft_path() -> Option<PathBuf> {
    let default_path = config_dir().unwrap().join(".minecraft");

    match default_path.exists() {
    	true => Some( default_path ),
    	false => None
    }
}

pub fn clear_configs(path: &PathBuf) -> std::io::Result<()> {
    remove_dir_all(path.join("mods"))
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
    
    if mods_path.exists() {
        if mods_path.metadata()?.len() > 0 {
            if backup_path.exists() {
                println!("\n[ {} ] Предыдущая резервная копия будет удалена. Для отмены нажмите Ctrl + C, или нажмите Enter для продолжения.", backup_path.display());
         
                let mut pause = String::new();
                std::io::stdin().read_line(&mut pause).unwrap();

                std::fs::remove_dir_all(&backup_path)?;
            }
            
            std::fs::rename(mods_path, backup_path)?;
        }
    }

    Ok(())
}
