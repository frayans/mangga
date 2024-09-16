use std::path::PathBuf;

use clap::Command;

use crate::core::filename::tidy_command;

mod filename;
use self::filename::filename;

pub fn run() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("filename", sub_matches)) => {
            let filename_command = sub_matches.subcommand();
            match filename_command {
                Some(("tidy", sub_matches)) => {
                    let path = sub_matches.get_one::<PathBuf>("path").unwrap();
                    // let dir_flag = sub_matches.get_flag("dir");

                    tidy_command(path);
                }
                Some((name, _)) => eprintln!("Unsupported subcommand `{name}`"),
                _ => unreachable!(),
            }
        }
        Some((name, _)) => eprintln!("Unsupported subcommand `{name}`"),
        _ => unreachable!(),
    }
}

fn cli() -> Command {
    Command::new("mangga")
        .about("A tool to manage manga collections")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(filename())
}
