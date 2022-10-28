use crate::utils::*;
use std::collections::HashMap;
use std::path::PathBuf;
use std::{env, fs};

const HOARDER_ENV: &'static str = "HOARDER";
const HOARDER_CONFIG: &'static str = "hoarder.json";

pub type Link = (PathBuf, PathBuf);

pub struct Config {
    pub links: Vec<Link>,
}

impl Config {
    pub fn new() -> Config {
        let root = Config::root();
        if Config::root().is_some() {
            panic!("Hoarder already exists at {:?}", root.unwrap());
        }

        let current_dir = &env_ext::current_dir();
        let config_path = Config::get_config_path(current_dir);
        if !config_path.exists() {
            fs::write(config_path, "{}").expect("Write Failed");
        }

        Config::from_dir(current_dir)
    }

    pub fn read() -> Config {
        let root = Config::root();
        if root.is_some() {
            Config::from_dir(root.as_ref().unwrap())
        } else {
            Config::from_dir(&env_ext::current_dir())
        }
    }

    fn check_health(links: &Vec<Link>) {
        let mut links: Vec<String> = links
            .iter()
            .map(|link| vec![link.0.clone(), link.1.clone()])
            .flatten()
            .map(|link| link.to_str().unwrap().to_string())
            .collect();

        links.sort_by(|a, b| a.len().cmp(&b.len()));

        for i in 0..links.len() {
            for j in i + 1..links.len() {
                if links[j].starts_with(&links[i]) {
                    panic!("Conflict files: {} and {}", &links[i], &links[j])
                }
            }
        }
    }

    fn from_dir(hoarder_dir: &PathBuf) -> Config {
        let config_path = &Config::get_config_path(hoarder_dir);
        if !config_path.exists() || !config_path.is_file() {
            panic!("Can not find {:?}", &config_path);
        }

        let content = fs::read_to_string(config_path).unwrap().to_string();
        let values: HashMap<String, HashMap<String, String>> =
            serde_json::from_str(&content).unwrap();

        let mut links: Vec<Link> = vec![];

        values.iter().for_each(|(key, map)| {
            let prefix = key;
            map.iter().for_each(|(src, dest)| {
                let mut src_path = PathBuf::from(hoarder_dir);
                src_path.push(prefix);
                src_path.push(src);
                let dest_path = path_ext::expand(dest.into());
                let dest_path = PathBuf::from(dest_path);
                links.push((src_path, dest_path));
            })
        });

        Config::check_health(&links);

        Config { links }
    }

    fn get_config_path(hoarder_dir: &PathBuf) -> PathBuf {
        let mut config_path = PathBuf::from(hoarder_dir);
        config_path.push(&HOARDER_CONFIG);
        config_path
    }

    fn root() -> Option<PathBuf> {
        env::var(&HOARDER_ENV).map(|root| PathBuf::from(&root)).ok()
    }
}
