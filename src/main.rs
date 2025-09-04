mod anime;
mod manga;
mod utils;

use std::{fs, io, path::PathBuf};
use utils::Result;

fn main() -> Result<()> {
    let files = get_files(".")?;

    if let Err(e) = anime::handle_anime(&files) {
        println!("Skip anime diff because of error: {e}");
    }

    if let Err(e) = manga::handle_manga(&files) {
        println!("Skip manga diff because of error: {e}");
    }

    println!("\nPress `Enter` to close this window.");
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(())
}

fn get_files(dir: &str) -> Result<Vec<PathBuf>> {
    let files = fs::read_dir(dir)?;

    let mut files: Vec<PathBuf> = files
        .into_iter()
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .collect();

    files.sort();
    files.reverse();

    Ok(files)
}
