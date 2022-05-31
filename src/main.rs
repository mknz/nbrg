use std::env;
use std::process;

use regex::Regex;
use walkdir::{DirEntry, WalkDir};

fn is_hidden(entry: &DirEntry) -> bool {
    let filename = entry.file_name().to_str().unwrap();
    if filename.eq(".") || filename.eq("..") {
        return false;
    };
    entry
        .file_name()
        .to_str()
        .map(|s| s.starts_with("."))
        .unwrap_or(false)
}

fn search_dir(dir: &str, re: &Regex, pattern: &str) {
    let walker = WalkDir::new(dir).into_iter();
    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        match entry {
            Ok(ent) => {
                if ent.file_type().is_file()
                    && ent
                        .file_name()
                        .to_str()
                        .map(|s| s.ends_with("ipynb"))
                        .unwrap_or(false)
                {
                    let filename = ent.path().to_str().unwrap().to_string();
                    nbrg::search(&filename, &re, pattern);
                }
            }
            Err(_) => {
                println!("{}: Invalid path", dir);
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.len() {
        1 => {
            println!("nbrg [OPTIONS] PATTERN [PATH ...]");
            process::exit(2);
        }
        _ => {
            let pattern = &args[1];
            let pattern_re = format!("({})", &args[1]);
            let re = Regex::new(&pattern_re).unwrap();
            match args.len() {
                2 => {
                    search_dir(".", &re, pattern);
                }
                _ => {
                    for arg in &args[2..] {
                        search_dir(&arg, &re, pattern);
                    }
                }
            }
            process::exit(0);
        }
    }
}
