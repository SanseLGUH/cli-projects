use std::io;

mod installer;
mod install_path;

#[tokio::main]
async fn main() -> io::Result<()> {
    let minecraft_path = install_path::find().display().to_string();
    println!("{} <- your minecraft path", minecraft_path);

    println!("\nWant to install DeathRun? [ press enter to install ]");

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let games = installer::games().await.unwrap();

    if let Some(game) = games.get("AMONGUS") {
        // installer::install(&game.path, "./nz-mini-game.zip").await;
        installer::unpack("./AMONGUS.zip", "./");
    }

    Ok(())
}
