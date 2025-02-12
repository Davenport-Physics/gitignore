
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;
use std::process::exit;

use clap::Parser;

enum GitIgnoreGenerator {
    VisualStudio,
    Flutter,
    None
}

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {

    /// Setup .gitignore for visual studio
    #[arg(short, long, default_value_t = false)]
    pub visual_studio: bool,

    /// Setup .gitignore for flutter
    #[arg(short, long, default_value_t = false)]
    pub flutter: bool,

    /// Overwrite the existing
    #[arg(short, long, default_value_t = true)]
    pub overwrite_existing: bool

}

impl Args {

    pub fn get_url(&self) -> Option<&str> {

        if (self.visual_studio) {
            return Some("https://raw.githubusercontent.com/github/gitignore/refs/heads/main/VisualStudio.gitignore");
        } else if (self.flutter) {
            return Some("https://raw.githubusercontent.com/github/gitignore/refs/heads/main/Flutter.gitignore");
        } else {
            None
        }

    }

    pub fn get_generator(&self) -> GitIgnoreGenerator {

        if self.visual_studio {
            GitIgnoreGenerator::VisualStudio
        } else if self.flutter {
            GitIgnoreGenerator::Flutter
        } else {
            GitIgnoreGenerator::None
        }

    }

}

fn main() {

    let args = Args::parse();
    let url = args.get_url();

    if url.is_none() {

        println!("You need to specify what .gitignore file to generate.");
        exit(1);

    }

    let gitignore = fetch_gitignore(url.unwrap());
    write_gitignore(gitignore, args.overwrite_existing);

}

fn fetch_gitignore(url: &str) -> String {

    let response = reqwest::blocking::get(url)
        .expect("Failed to fetch gitignore");

    response.text()
        .expect("Failed to read response text")

}

fn write_gitignore(gitignore: String, overwrite_existing: bool) {

    let path = Path::new(".gitignore");

    if !path.exists() || overwrite_existing {
        fs::write(path, gitignore).unwrap();
        return;
    }

    OpenOptions::new()
        .append(true)
        .open(path)
        .unwrap()
        .write(gitignore.as_bytes())
        .expect("Failed to append to gitignore file");

}