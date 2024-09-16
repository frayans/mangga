use std::path::{Path, PathBuf};

use clap::Command;

use crate::core::filename::{filter_extensions, parse_filename, FilenameOptions};

mod filename;
use self::filename::filename;

pub fn run() {
    let matches = cli().get_matches();

    match matches.subcommand() {
        Some(("filename", sub_matches)) => {
            let filename_command = sub_matches.subcommand().unwrap();
            match filename_command {
                ("tidy", sub_matches) => {
                    let path = sub_matches.get_one::<PathBuf>("path").unwrap();
                    // let dir_flag = sub_matches.get_flag("dir");

                    tidy_command(path);
                }
                (name, _) => eprintln!("Unsupported subcommand `{name}`"),
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

fn tidy_command(path: &Path) {
    let filtered_iter = filter_extensions(path);
    for file in filtered_iter {
        let parsed = parse_filename(file.file_name());
        let options = FilenameOptions::new(parsed.expect("invalid filename format"))
            .with_title()
            .with_volume()
            .with_medium()
            .with_creator()
            .with_extension()
            .build();

        println!("{} -> {}", file.file_name().to_str().unwrap(), options);
    }
}
