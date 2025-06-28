mod installer;
mod local_utils;

use std::{ io::Write, path::PathBuf };
use tokio;

fn input(prompt: &str) -> String {
    print!("{}", prompt);
    std::io::stdout().flush().unwrap();
    let mut buf = String::new();
    std::io::stdin().read_line(&mut buf).unwrap();
    buf.trim().to_string()
}

async fn installation(minecraft_path: PathBuf, game_url: &str) {
    local_utils::backup(&minecraft_path).expect("Failed to create Backup");

    let _ = local_utils::clear_configs(&minecraft_path);
    installer::install(game_url, "nz_game.zip").await.expect("Failed to install");

    println!("Распаковываем файл...");
    local_utils::unpack("nz_game.zip", &minecraft_path).expect("Failed to extract file");
    let _ = std::fs::remove_file("nz_game.zip");
}

fn custom_path() -> PathBuf { 
    let custom_path = input("Введите свой путь: ");
    let custom_path_formated = std::path::Path::new(&custom_path).to_path_buf();

    match custom_path_formated.exists() {
        true => {
            println!("Вы указали путь: {}", custom_path);
            custom_path_formated
        }
        false => {
            println!("Этого путь не существует!");
            std::process::exit(1);
        }
    }
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let path: PathBuf = match local_utils::default_minecraft_path() {
        Some(mut path) => {
            println!("\n[ Путь по умолчанию: {:?} ]", path);

            if input("Хотите указать свой путь? (введите 'y' для да): ").to_lowercase() == "y" {
                path = custom_path();
            }
            
            path
        }
        None => {
            println!("Не удалось определить путь установки Minecraft.");
            custom_path()
        }
    };

    if input("Распаковать предустановленную мини-игру? (введите 'y' для Да): ").to_lowercase() == "y" {
        let custom_path = input("Укажите путь для распаковки: ");

        local_utils::backup(&path)?;
        local_utils::unpack(&custom_path, &path)?;

        println!("Распаковка завершена успешно!");
        return Ok(());
    }

    let games = installer::games().await.expect("Ошибка при получении списка игр");

    println!("\nСписок доступных игр:");
    for (name, data) in &games {
        println!(" - {} :: описание: {}", name, data.description);
    }

    let game_choice = input("\nВведите название игры из списка: ");

    match games.get(&game_choice) {
        Some(game) => {
            println!("Путь к выбранной игре: {:?} :: установка может занять от 20 до 30 секунд", game.path);
            installation(path, &game.path).await;
            input("Установка завершена!");
        }
        None => println!("Игра с таким названием не найдена.")
    }

    Ok(())
}
