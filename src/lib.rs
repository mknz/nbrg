use std::fs;

use colored::*;
use regex::Regex;
use serde_json::Value;

pub fn search(filename: &str, re: &Regex, pattern: &str) {
    let data = fs::read_to_string(filename).expect("Unable to read file");

    // Parse json
    let v: Value = match serde_json::from_str(&data) {
        Ok(v) => v,
        Err(e) => {
            println!("{}: {:?}", filename, e);
            return;
        }
    };

    let mut is_first_match = true;
    let mut n_cell = 1;
    let empty_vec = Vec::new();

    for cell in v["cells"].as_array().unwrap_or(&empty_vec).iter() {
        if cell["cell_type"] == "code" {
            let mut n_line = 1;
            let mut is_first_match_cell = true;
            for line in cell["source"].as_array().unwrap_or(&empty_vec).iter() {
                let mut line_str = line.as_str().unwrap_or("").to_string();

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
