use clap::Command;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeMap, fs::OpenOptions};

#[derive(Serialize, Deserialize)]
struct Data {
    entries: BTreeMap<String, String>,
}

fn main() {
    let matches = Command::new("dm")
        .version("1.0")
        .about("Your personal directory door man")
        .arg_required_else_help(true)
        .subcommands([
            Command::new("add").about("Adds a new directory"),
            Command::new("ls").about("Prints all directories"),
        ])
        .get_matches();

    match matches.subcommand() {
        Some(("add", _)) => {
            let path = std::env::current_dir().unwrap();
            let key = path
                .to_str()
                .unwrap()
                .split('/')
                .last()
                .unwrap()
                .to_string();
            let value = path.to_str().unwrap().to_string();
            let mut data = load_data();
            data.entries.insert(key, value);
            save_data(&data);
        }
        Some(("ls", _)) => {
            let data = load_data();
            for (key, value) in &data.entries {
                println!("{} -> {}", key, value);
            }
        }
        _ => unreachable!(),
    }
}

fn load_data() -> Data {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("target/dm.json")
        .unwrap();
    serde_json::from_reader(&file).unwrap_or_else(|_| Data {
        entries: BTreeMap::new(),
    })
}

fn save_data(data: &Data) {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open("target/dm.json")
        .unwrap();
    serde_json::to_writer_pretty(&file, data).unwrap();
}
