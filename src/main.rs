use clap::Command;

fn main() {
    let matches = Command::new("dm")
        .version("1.0")
        .about("Your personal directory door man")
        .arg_required_else_help(true)
        .subcommands([
            Command::new("pwd").about("Prints current directory"),
            Command::new("base").about("Prints base directory"),
        ])
        .get_matches();

    match matches.subcommand() {
        Some(("pwd", _)) => {
            println!("{}", std::env::current_dir().unwrap().to_str().unwrap());
        }
        Some(("base", _)) => println!(
            "{}",
            std::env::current_dir()
                .unwrap()
                .to_str()
                .unwrap()
                .split('/')
                .last()
                .unwrap()
        ),
        _ => unreachable!(),
    }
}
