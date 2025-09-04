use flate2::read::GzDecoder;
use std::io::Read;
use std::{collections::HashMap, fs::File, path::Path};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub fn gz_to_str(path: &Path) -> Result<String> {
    let mut content = String::new();
    let file = File::open(path)?;
    let mut dec = GzDecoder::new(file);
    dec.read_to_string(&mut content)?;
    Ok(content)
}

pub fn compare<'a, T: PartialEq>(
    map1: &'a HashMap<u64, &'a T>,
    map2: &'a HashMap<u64, &'a T>,
) -> HashMap<u64, &'a T> {
    let mut result = HashMap::new();

    for (key, value) in map1.iter() {
        let other = map2.get(key).cloned();

        let not_equal = other.map(|o| o != *value).unwrap_or(true);

        if not_equal {
            let entry = result.entry(*key);
            entry.insert_entry(*value);
        }
    }

    result
}

// TODO: remove Clone
pub fn uniq<T: Clone>(diff1: HashMap<u64, &T>, diff2: HashMap<u64, &T>) -> Vec<T> {
    let mut result = Vec::new();

    for (key, value) in diff1.into_iter() {
        if let Some(_other) = diff2.get(&key) {
            let item = value.clone();
            result.push(item);
        }
    }

    result
}

pub fn path_matches<F: Fn(&str) -> bool>(path: &Path, f: F) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(f)
        .unwrap_or(false)
}
