use std::path::PathBuf;

use clap::Command;
use walkdir::WalkDir;

use crate::core::filename::{has_supported_extension, parse_filename, FilenameOptions};

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
                    let has_title = sub_matches.get_flag("title");
                    let has_vol = sub_matches.get_flag("vol");
                    let has_year = sub_matches.get_flag("year");
                    let has_medium = sub_matches.get_flag("medium");
                    let has_creator = sub_matches.get_flag("creator");
                    let has_ext = sub_matches.get_flag("ext");

                    let mut opts = FilenameOptions::new();
                    if has_title {
                        opts.with_title();
                    }
                    if has_vol {
                        opts.with_volume();
                    }
                    if has_year {
                        opts.with_year();
                    }
                    if has_medium {
                        opts.with_medium();
                    }
                    if has_creator {
                        opts.with_creator();
                    }
                    if has_ext {
                        opts.with_extension();
                    }

                    for file in WalkDir::new(path)
                        .into_iter()
                        .filter_map(|f| f.ok())
                        .filter(has_supported_extension)
                    {
                        let parsed = parse_filename(file.file_name());
                        let fname = opts.build(parsed.expect("Invalid filename format"));

                        println!("{} -> {}", file.file_name().to_str().unwrap(), fname);
                    }
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
