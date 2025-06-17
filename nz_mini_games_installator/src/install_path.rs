use std::{io, path::PathBuf};
use dirs::config_dir;

enum Launcher {
	// Official, Tlauncher, Tlauncher 
	DefaultPath,
	// Prisma, KLauncher, tc 
	CustomPath
}

fn define_launcher() -> Launcher {
	// let mut entries = fs::read_dir(config_dir().unwrap()).unwrap()
    //     .map(|res| res.map(|e| e.path()))
    //     .collect::<Result<Vec<_>, io::Error>>().unwrap();

   	Launcher::DefaultPath
}

pub fn find() -> PathBuf {
	match define_launcher() {
		Launcher::DefaultPath => dot_minecraft(),
		Launcher::CustomPath => todo!()
	}
}

fn dot_minecraft() -> PathBuf {
	let mut romaning = config_dir().unwrap();
	romaning.push(".minecraft");
	romaning
}