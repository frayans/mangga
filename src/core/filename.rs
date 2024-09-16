use std::ffi::OsStr;

use regex::Regex;
use walkdir::DirEntry;

pub fn parse_filename(input: &OsStr) -> Option<Filename> {
    let re = Regex::new(r"^(?<title>.*?).(?<vol>v\d+).(?<year>\(\d{4}\)).(?<medium>\(Digital\)).(?<creator>\(\w+\)).(?<extension>.*)$").unwrap();
    let matched = re.captures(input.to_str()?)?;

    let title = matched.name("title")?.as_str();
    let volume = matched.name("vol")?.as_str();
    let year = matched.name("year")?.as_str();
    let medium = matched.name("medium")?.as_str();
    let creator = matched.name("creator")?.as_str();
    let extension = matched.name("extension")?.as_str();

    Some(Filename {
        title,
        volume,
        year,
        medium,
        creator,
        extension,
    })
}

pub fn has_supported_extension(entry: &DirEntry) -> bool {
    has_extension(entry, "cbz")
        || has_extension(entry, "zip")
        || has_extension(entry, "cbr")
        || has_extension(entry, "rar")
        || has_extension(entry, "cb7")
        || has_extension(entry, "7z")
}

fn has_extension(entry: &DirEntry, extension: &str) -> bool {
    entry
        .path()
        .extension()
        .map(|it| it == extension)
        .unwrap_or(false)
}

#[derive(Debug)]
pub struct Filename<'a> {
    pub title: &'a str,
    pub volume: &'a str,
    pub year: &'a str,
    pub medium: &'a str,
    pub creator: &'a str,
    pub extension: &'a str,
}

pub struct FilenameOptions {
    with_title: bool,
    with_volume: bool,
    with_year: bool,
    with_medium: bool,
    with_creator: bool,
    with_extension: bool,
}

impl FilenameOptions {
    pub fn new() -> Self {
        Self {
            with_title: Default::default(),
            with_volume: Default::default(),
            with_year: Default::default(),
            with_medium: Default::default(),
            with_creator: Default::default(),
            with_extension: Default::default(),
        }
    }

    pub fn with_title(&mut self) -> &mut Self {
        self.with_title = true;
        self
    }

    pub fn with_volume(&mut self) -> &mut Self {
        self.with_volume = true;
        self
    }

    pub fn with_year(&mut self) -> &mut Self {
        self.with_year = true;
        self
    }

    pub fn with_medium(&mut self) -> &mut Self {
        self.with_medium = true;
        self
    }

    pub fn with_creator(&mut self) -> &mut Self {
        self.with_creator = true;
        self
    }

    pub fn with_extension(&mut self) -> &mut Self {
        self.with_extension = true;
        self
    }

    pub fn build(&self, filename: Filename<'_>) -> String {
        let mut s = String::new();

        if self.with_title {
            s.push(' ');
            s.push_str(&filename.title);
        }

        if self.with_volume {
            s.push(' ');
            s.push_str(&filename.volume);
        }

        if self.with_year {
            s.push(' ');
            s.push_str(&filename.year);
        }

        if self.with_medium {
            s.push(' ');
            s.push_str(&filename.medium);
        }

        if self.with_creator {
            s.push(' ');
            s.push_str(&filename.creator);
        }

        if self.with_extension {
            s.push('.');
            s.push_str(&filename.extension);
        }

        s.trim().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::OsStr;

    #[test]
    fn test_parse_filename() {
        let test_str = "Akane-banashi v01 (2023) (Digital) (1r0n).cbz";
        let parsed = parse_filename(OsStr::new(test_str)).unwrap();
        assert_eq!(parsed.title, "Akane-banashi");
        assert_eq!(parsed.volume, "v01");
        assert_eq!(parsed.year, "(2023)");
        assert_eq!(parsed.medium, "(Digital)");
        assert_eq!(parsed.creator, "(1r0n)");
        assert_eq!(parsed.extension, "cbz");
    }

    #[test]
    fn test_filename_options() {
        let test_str = "Akane-banashi v01 (2023) (Digital) (1r0n).cbz";
        let parsed = parse_filename(OsStr::new(test_str)).unwrap();
        let options = FilenameOptions::new()
            .with_title()
            .with_volume()
            .build(parsed);
        assert_eq!(&options, "Akane-banashi v01");
    }

    #[test]
    fn test_filename_options_extension() {
        let test_str = "Akane-banashi v01 (2023) (Digital) (1r0n).cbz";
        let parsed = parse_filename(OsStr::new(test_str)).unwrap();
        let options = FilenameOptions::new()
            .with_title()
            .with_volume()
            .with_extension()
            .build(parsed);
        assert_eq!(&options, "Akane-banashi v01.cbz");
    }
}
