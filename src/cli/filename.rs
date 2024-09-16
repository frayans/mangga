use std::path::PathBuf;

use clap::{arg, value_parser, Command};

pub fn filename() -> Command {
    Command::new("filename")
        .about("Work with filenames")
        .arg_required_else_help(true)
        .subcommand(tidy())
}

fn tidy() -> Command {
    Command::new("tidy")
        .about("Tidy up filenames")
        .arg(arg!(dir: -d --dir "Accept a directory"))
        .arg(arg!(path: <PATH> "Path of file to be changed").value_parser(value_parser!(PathBuf)))
}
