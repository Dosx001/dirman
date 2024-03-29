use clap::{value_parser, Arg, ArgAction, Command};
use clap_complete::{generate, Generator, Shell};
use serde::{Deserialize, Serialize};
use std::{
    collections::BTreeMap,
    fs::{self, OpenOptions},
    path::PathBuf,
};

#[derive(Serialize, Deserialize)]
struct Json {
    entries: BTreeMap<String, String>,
}

fn main() {
    let matches = build_cli().get_matches();
    if let Some(generator) = matches.get_one::<Shell>("generator").copied() {
        let mut cmd = build_cli();
        eprintln!("Generating completion file for {generator}...");
        print_completions(generator, &mut cmd);
    }
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
            println!("{:?} -> {:?}", key, value);
            data.entries.insert(key, value);
            save_data(&data);
        }
        Some(("rm", args)) => {
            let key = args.subcommand().unwrap().0;
            let mut data = load_data();
            if data.entries.remove(key).is_some() {
                save_data(&data);
                println!("Remove {}", key);
            }
        }
        Some(("ls", _)) => {
            let data = load_data();
            for (key, value) in &data.entries {
                println!("{} -> {}", key, value);
            }
        }
        Some(("_get", args)) => {
            let key = args.subcommand().unwrap().0;
            println!("{}", load_data().entries.get(key).unwrap());
        }
        Some((ext, _)) => {
            if move_directory(ext) {
                println!("Value not found!");
            }
        }
        _ => {
            println!("Invalid command");
        }
    }
}

fn build_cli() -> Command {
    Command::new("dm")
        .version("1.0")
        .about("Your personal directory door man")
        .arg_required_else_help(true)
        .allow_external_subcommands(true)
        .subcommands([
            Command::new("add").about("Adds a new directory"),
            Command::new("rm")
                .about("Removes a directory")
                .allow_external_subcommands(true),
            Command::new("ls").about("Prints all directories"),
            Command::new("_get")
                .hide(true)
                .allow_external_subcommands(true),
        ])
        .arg(
            Arg::new("generator")
                .long("generate")
                .action(ArgAction::Set)
                .value_parser(value_parser!(Shell)),
        )
}

fn print_completions<G: Generator>(gen: G, cmd: &mut Command) {
    generate(gen, cmd, cmd.get_name().to_string(), &mut std::io::stdout());
}

fn move_directory(base: &str) -> bool {
    let data = load_data();
    for (key, value) in &data.entries {
        if key == base {
            println!("{} -> {}", key, value);
            let output = std::process::Command::new("cd")
                .arg(value)
                .output()
                .expect("Failed to move command");

            println!("{}", String::from_utf8_lossy(&output.stdout));
            return false;
        }
    }
    true
}

fn load_data() -> Json {
    let mut dir_path = PathBuf::new();
    dir_path.push(dirs::home_dir().expect("Failed to get home directory"));
    dir_path.push(".dm");
    if !dir_path.exists() {
        fs::create_dir_all(&dir_path).unwrap();
    }
    let path = dir_path.join("data.json");
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .unwrap();
    serde_json::from_reader(&file).unwrap_or_else(|_| Json {
        entries: BTreeMap::new(),
    })
}

fn save_data(data: &Json) {
    let mut dir_path = PathBuf::new();
    dir_path.push(dirs::home_dir().expect("Failed to get home directory"));
    dir_path.push(".dm");
    if !dir_path.exists() {
        fs::create_dir_all(&dir_path).unwrap();
    }
    let path = dir_path.join("data.json");
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(path)
        .unwrap();
    serde_json::to_writer_pretty(&file, data).unwrap();
}
