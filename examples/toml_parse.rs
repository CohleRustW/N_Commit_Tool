use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;

#[derive(Deserialize, Debug)]
struct Project {
    name: String,
    version: String,
    author: String,
    dependencies: Vec<String>,
}

fn main() {
    let contents = include_str!("../fixfures/test.toml");
    println!("{}", contents);
    let config: HashMap<String, Project> = toml::from_str(&contents).expect("Failed to parse file");
    println!("{:#?}", config);
    for (c, m) in config.iter() {
        let re: Regex = Regex::new(m.author.as_str()).unwrap();
        println!("{} {}", c, re.is_match("V1.0.0-rc"));
    }
}

pub fn get_current_path() -> String {
    let current_path = std::env::current_dir().expect("Failed to get current path");
    let current_path = current_path.to_str().unwrap();
    current_path.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_current_path() {
        let current_path = get_current_path();
        println!("{}", current_path);
    }
}
