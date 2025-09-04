use quick_xml::de::from_str;
use serde::Deserialize;
use std::{fs, path::PathBuf};
use utils::Result;

use crate::utils;

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub enum Status {
    Reading,
    Completed,
    #[serde(rename = "On-Hold")]
    OnHold,
    Dropped,
    #[serde(rename = "Plan to Read")]
    Planned,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct Manga {
    id: u64,
    title: String,
    status: Status,
    score: u8,
    chapters_read: u32,
    volumes_read: u32,
}

impl PartialEq for Manga {
    fn eq(&self, other: &Self) -> bool {
        if self.status == Status::Unknown {
            return false;
        }

        if self.status == Status::Completed {
            return self.id == other.id
                && self.status == other.status
                && self.score == other.score
                && self.volumes_read == other.volumes_read;
        }

        self.id == other.id
            && self.status == other.status
            && self.score == other.score
            && self.volumes_read == other.volumes_read
            && self.chapters_read == other.chapters_read
    }
}

#[derive(Debug, Deserialize)]
pub struct Root<T> {
    pub manga: Vec<T>,
}

#[derive(Debug, Deserialize)]
pub struct MalManga {
    manga_mangadb_id: u64,
    manga_title: String,
    my_status: Status,
    my_score: u8,
    my_read_chapters: u32,
    my_read_volumes: u32,
}

#[derive(Debug, Deserialize)]
pub struct ShikiManga {
    manga_mangadb_id: u64,
    series_title: String,
    my_status: Status,
    my_score: u8,
    my_read_chapters: u32,
    my_read_volumes: u32,
}

impl From<ShikiManga> for Manga {
    fn from(value: ShikiManga) -> Self {
        Self {
            id: value.manga_mangadb_id,
            title: value.series_title,
            status: value.my_status,
            score: value.my_score,
            chapters_read: value.my_read_chapters,
            volumes_read: value.my_read_volumes,
        }
    }
}

impl From<MalManga> for Manga {
    fn from(value: MalManga) -> Self {
        Self {
            id: value.manga_mangadb_id,
            title: value.manga_title,
            status: value.my_status,
            score: value.my_score,
            chapters_read: value.my_read_chapters,
            volumes_read: value.my_read_volumes,
        }
    }
}

fn get_mal_mangas(paths: &[PathBuf]) -> Result<Vec<Manga>> {
    let path = paths
        .iter()
        .find(|path| utils::path_matches(path, |name| name.starts_with("mangalist_")))
        .ok_or("mal manga file not found")?;

    let content = utils::gz_to_str(path)?;
    let doc: Root<MalManga> = from_str(&content)?;
    let mangas: Vec<Manga> = doc.manga.into_iter().map(From::from).collect();

    Ok(mangas)
}

fn get_shiki_mangas(paths: &[PathBuf]) -> Result<Vec<Manga>> {
    let path = paths
        .iter()
        .find(|path| utils::path_matches(path, |name| name.ends_with("_mangas.xml")))
        .ok_or("shiki manga file not found")?;

    let content = fs::read_to_string(path)?;
    let doc: Root<ShikiManga> = from_str(&content)?;
    let mangas: Vec<Manga> = doc.manga.into_iter().map(From::from).collect();

    Ok(mangas)
}

pub fn handle_manga(paths: &[PathBuf]) -> Result<()> {
    let shiki = get_shiki_mangas(paths)?;
    let mal = get_mal_mangas(paths)?;
    let diff = get_diff_manga(&mal, &shiki);

    print_diff(&diff);

    Ok(())
}

fn get_diff_manga(list1: &[Manga], list2: &[Manga]) -> Vec<Manga> {
    let map1 = list1.iter().map(|a| (a.id, a)).collect();
    let map2 = list2.iter().map(|a| (a.id, a)).collect();

    let diff1 = utils::compare(&map1, &map2);
    let diff2 = utils::compare(&map2, &map1);

    utils::uniq(diff1, diff2)
}

fn print_diff(list: &[Manga]) {
    if list.is_empty() {
        println!("No Manga diff!");
        return;
    }

    println!("Manga diff:");

    for manga in list {
        println!("{}: {}", manga.id, manga.title)
    }
}
