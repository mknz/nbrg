use std::env;
use std::fs;
use std::path::Path;
use std::process;

use colored::*;
use regex::Regex;
use serde_json::Value;
use walkdir::WalkDir;

fn search(filename: &String, re: &Regex, pattern: &String) {
    let data = fs::read_to_string(filename).expect("Unable to read file");

    // Parse json
    let v: Value = serde_json::from_str(&data).unwrap();

    let mut is_first_match = true;
    let mut n_cell = 1;
    for cell in v["cells"].as_array().unwrap().iter() {
        if cell["cell_type"] == "code" {
            let mut n_line = 1;
            let mut is_first_match_cell = true;
            for line in cell["source"].as_array().unwrap().iter() {
                let mut line_str = line.as_str().unwrap().to_string();

                // Truncate trailing newline
                let len = line_str.trim_end_matches(&['\r', '\n'][..]).len();
                line_str.truncate(len);

                if re.is_match(line_str.as_str()) {
                    // Display filename only once
                    if is_first_match {
                        println!("{}", filename.purple());
                        is_first_match = false;
                    }
                    if is_first_match_cell {
                        let cell_str = format!("cell {}", n_cell);
                        println!("{}", cell_str.cyan());
                        is_first_match_cell = false;
                    }
                    let matched = format!("{}", pattern.red().bold());
                    println!(
                        "{}: {}",
                        n_line.to_string().green(),
                        re.replace_all(line_str.as_str(), matched.as_str()),
                    );
                }
                n_line += 1;
            }
        }
        n_cell += 1;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        println!("nbrg PATTERN [PATH ...]");
        process::exit(2);
    }

    let pattern = &args[1];
    let pattern_re = format!("({})", &args[1]).to_owned();
    let re = Regex::new(&pattern_re).unwrap();

    if args.len() == 2 {
        let walker = WalkDir::new(".").into_iter();
        for entry in walker {
            let entry = entry.unwrap();
            if entry.file_type().is_file()
                && entry
                    .file_name()
                    .to_str()
                    .map(|s| s.ends_with("ipynb"))
                    .unwrap_or(false)
            {
                let filename = entry.path().to_str().unwrap().to_string();
                search(&filename, &re, pattern);
            }
        }
        process::exit(0);
    }

    let filenames = &args[2..];
    for filename in filenames.iter() {
        if !Path::new(filename).exists() {
            println!("No such file or directory");
            process::exit(2);
        }
        search(filename, &re, pattern)
    }
    process::exit(0);
}
