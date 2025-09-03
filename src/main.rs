use std::{io, path::PathBuf};

mod anime;
mod manga;
mod utils;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let files = get_files(".")?;

    anime::handle_anime(&files)?;
    manga::handle_manga(&files)?;

    println!("\nPress `Enter` to close this window.");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(())
}

fn get_files(dir: &str) -> Result<Vec<PathBuf>, Box<dyn std::error::Error>> {
    let files = std::fs::read_dir(dir)?;

    let mut files: Vec<PathBuf> = files
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .collect();

    files.sort();
    files.reverse();

    Ok(files)
}
