use std::env;
use std::path::PathBuf;

pub fn home_dir() -> String {
    env::var("HOME").unwrap()
}

pub fn current_dir() -> PathBuf {
    env::current_dir().unwrap()
}
