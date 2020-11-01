use std::env;
use std::path::Path;
use std::process;

use regex::Regex;
use walkdir::WalkDir;

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
                nbrg::search(&filename, &re, pattern);
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
        nbrg::search(filename, &re, pattern)
    }
    process::exit(0);
}
