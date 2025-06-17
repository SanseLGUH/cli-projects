use std::io;

mod installer;
mod install_path;

#[tokio::main]
async fn main() -> io::Result<()> {
    println!("{:?}", installer::unpack("./among.rar", "./among/"));

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(())
}
