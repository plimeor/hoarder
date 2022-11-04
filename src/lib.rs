mod config;
mod utils;

use config::Config;
use std::fs;
use utils::*;

pub enum Command {
    Init,
    Collect,
    Restore,
}

impl Command {
    pub fn new(args: Vec<String>) -> Command {
        let command = args.get(1).unwrap_or(&String::from("")).clone();
        match command.as_str() {
            "init" => Command::Init,
            "collect" => Command::Collect,
            "restore" => Command::Restore,
            _ => panic!("Missing command name: init | collect | restore"),
        }
    }
}

pub fn run(command: Command) {
    match command {
        Command::Init => {
            init();
        }
        Command::Collect => {
            collect().unwrap();
        }
        Command::Restore => {
            restore().unwrap();
        }
    };
}

fn init() {
    Config::new();
}

fn collect() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::read();
    for (src, dest) in config.links.iter() {
        println!("Collecting {:?}", dest);

        if dest.is_symlink() {
            let target = &fs::read_link(dest)?;
            if target == src {
                continue;
            } else {
                panic!(
                    "Destination {:?} is link to {:?}, instead {:?}",
                    dest, target, src
                );
            }
        } else if dest.exists() {
            fs_ext::copy(dest, src)?;
            fs_ext::remove(dest)?;
        }

        fs_ext::ensure_dir(dest)?;
        fs_ext::symlink(src, dest)?;
    }

    Ok(())
}

fn restore() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::read();
    for (src, dest) in config.links.iter() {
        if src.exists() {
            println!("Restoring {:?}", src);
            fs_ext::remove(dest)?;
            fs_ext::copy(src, dest)?;
        }
    }
    Ok(())
}
