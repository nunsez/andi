use crate::utils;
use quick_xml::de::from_str;
use serde::Deserialize;
use std::{fs, path::PathBuf};

#[derive(Debug, PartialEq, Clone, Deserialize)]
pub enum Status {
    Watching,
    Completed,
    #[serde(rename = "On-Hold")]
    OnHold,
    Dropped,
    #[serde(rename = "Plan to Watch")]
    Planned,
    Unknown,
}

#[derive(Debug, Clone)]
pub struct Anime {
    id: u64,
    title: String,
    status: Status,
    score: u8,
    episodes_watched: u32,
}

impl PartialEq for Anime {
    fn eq(&self, other: &Self) -> bool {
        if self.status == Status::Unknown {
            return false;
        }

        self.id == other.id
            && self.status == other.status
            && self.score == other.score
            && self.episodes_watched == other.episodes_watched
    }
}

#[derive(Debug, Deserialize)]
pub struct Root<T> {
    pub anime: Vec<T>,
}

#[derive(Debug, Deserialize)]
pub struct MalAnime {
    series_animedb_id: u64,
    series_title: String,
    my_status: Status,
    my_score: u8,
    my_watched_episodes: u32,
}

#[derive(Debug, Deserialize)]
pub struct ShikiAnime {
    pub series_animedb_id: u64,
    pub series_title: String,
    pub my_status: Status,
    pub my_score: u8,
    pub my_watched_episodes: u32,
}

impl From<ShikiAnime> for Anime {
    fn from(value: ShikiAnime) -> Self {
        Self {
            id: value.series_animedb_id,
            title: value.series_title,
            status: value.my_status,
            score: value.my_score,
            episodes_watched: value.my_watched_episodes,
        }
    }
}

impl From<MalAnime> for Anime {
    fn from(value: MalAnime) -> Self {
        Self {
            id: value.series_animedb_id,
            title: value.series_title,
            status: value.my_status,
            score: value.my_score,
            episodes_watched: value.my_watched_episodes,
        }
    }
}

fn get_mal_animes(paths: &[PathBuf]) -> Result<Vec<Anime>, Box<dyn std::error::Error>> {
    let path = paths
        .iter()
        .find(|path| utils::path_matches(path, |name| name.starts_with("animelist_")))
        .ok_or("mal anime file not found")?;

    let content = utils::gz_to_str(path)?;
    let doc: Root<MalAnime> = from_str(&content)?;
    let animes: Vec<Anime> = doc.anime.into_iter().map(From::from).collect();

    Ok(animes)
}

fn get_shiki_animes(paths: &[PathBuf]) -> Result<Vec<Anime>, Box<dyn std::error::Error>> {
    let path = paths
        .iter()
        .find(|path| utils::path_matches(path, |name| name.ends_with("_animes.xml")))
        .ok_or("shiki anime file not found")?;

    let content = fs::read_to_string(path)?;
    let doc: Root<ShikiAnime> = from_str(&content)?;
    let animes: Vec<Anime> = doc.anime.into_iter().map(From::from).collect();

    Ok(animes)
}

pub fn handle_anime(paths: &[PathBuf]) -> Result<(), Box<dyn std::error::Error>> {
    let shiki = get_shiki_animes(paths)?;
    let mal = get_mal_animes(paths)?;
    let diff = get_diff_anime(&mal, &shiki);

    print_diff(&diff);

    Ok(())
}

fn get_diff_anime(list1: &[Anime], list2: &[Anime]) -> Vec<Anime> {
    let map1 = list1.iter().map(|a| (a.id, a)).collect();
    let map2 = list2.iter().map(|a| (a.id, a)).collect();

    let diff1 = utils::compare(&map1, &map2);
    let diff2 = utils::compare(&map2, &map1);

    utils::uniq(diff1, diff2)
}

fn print_diff(list: &[Anime]) {
    if list.is_empty() {
        println!("No Anime diff!");
        return;
    }

    println!("Anime diff:");

    for anime in list {
        println!("{}: {}", anime.id, anime.title)
    }
}
