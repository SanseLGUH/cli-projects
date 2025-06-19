use std::io;

mod installer;
mod install_path;

use std::io::Write;
use tokio;

fn input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

async fn installation(minecraft_path: std::path::PathBuf, game_url: &str) {
    let _ = install_path::clear_mods(&minecraft_path);
    installer::install(game_url, "nz_game.zip").await.expect("Failed to install");

    println!("Распаковываем файл...");
    installer::unpack("nz_game.zip", &minecraft_path.display().to_string()).expect("Failed to extract file");
    let _ = std::fs::remove_file("nz_game.zip");
}

#[tokio::main]
async fn main() -> io::Result<()> {
    match install_path::default_minecraft_path() {
        Some(mut path) => {
            println!("\n[ Путь по умолчанию: {:?} ]", path);

            let choice = input("Хотите указать свой путь? (введите 'y' для да): ");

            if choice.to_lowercase() == "y" {
                let custom_path = input("Введите свой путь: ");
                
                let cp_to_path = std::path::Path::new(&custom_path); 

                match cp_to_path.exists() {
                    true => {
                        println!("Вы указали путь: {}", custom_path);        
                        path = cp_to_path.to_path_buf();
                    }
                    false => {
                        println!("Этого путь не существует!");
                        std::process::exit(1);
                    }
                }
            }

            let games = installer::games().await.expect("Ошибка при получении списка игр");

            println!("\nСписок доступных игр:");
            for (name, data) in &games {
                println!(" - {} :: описание: {}", name, data.description);
            }

            let game_choice = input("\nВведите название игры из списка: ");

            match games.get(&game_choice) {
                Some(game) => {
                    println!("Путь к выбранной игре: {:?}", game.path);

                    installation(path, &game.path).await;

                    input("Установка завершена!");
                }
                None => {
                    println!("Игра с таким названием не найдена.");
                }
            }            
        }
        None => {
            println!("Не удалось определить путь установки Minecraft.");
        }
    }

    Ok(())
}
