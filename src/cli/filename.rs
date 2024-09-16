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
        .arg(arg!(title: -t --with_title "Include <title> in resulting filename(s)"))
        .arg(arg!(vol: -v --with_volume "Include <volume> in resulting filename(s)"))
        .arg(arg!(year: -y --with_year "Include <year> in resulting filename(s)"))
        .arg(arg!(medium: -m --with_medium "Include <medium> in resulting filename(s)"))
        .arg(arg!(creator: -c --with_creator "Include <creator> in resulting filename(s)"))
        .arg(arg!(ext: -e --with_extension "Include <extension> in resulting filename(s)"))
        .arg(arg!(path: <PATH> "Path of file to be changed").value_parser(value_parser!(PathBuf)))
}
