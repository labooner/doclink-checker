use std::{env, fs};
use std::fs::DirEntry;
use std::path::PathBuf;
use regex::Regex;
use checker::{TestResult};
use colored::*;

mod checker;

fn main() {
    let mut current_dir = env::current_dir().unwrap();
    current_dir.push("test_data");

    scan_dir(current_dir);
}

fn scan_dir(dir: PathBuf) {
    match fs::read_dir(dir.clone()) {
        Ok(entries) => {
            for entry in entries {
                let entry = entry.unwrap();
                handle_dir_entry(entry);
            }
        },
        Err(e) => println!("Failed to read dir: {} - {}", dir.to_str().unwrap(), e),
    };    
}

fn handle_dir_entry(entry: DirEntry) {
    let file_type = entry.file_type().expect("failed to get file_type");

    if file_type.is_dir() {
        scan_dir(entry.path());
    }  else {
        read_file(entry.path());
    }
}

fn read_file(path: PathBuf) {
    let contents = fs::read_to_string(path.as_path()).expect("failed to read file");

    let re = Regex::new(r#"((http|ftp|https)://([\w_-]+(?:(?:\.[\w_-]+)+))([\w.,@?^=%&:/~+#-]*[\w@?^=%&/~+#-])?)"#).unwrap();

    for str_match in re.captures_iter(&contents) {
        let str_match = str_match.get(1).expect("failed to get first match out of a Match");
        let url = str_match.as_str();
        handle_test_result(path.to_str().unwrap(), url, checker::test_url(url));
    }
}

/// Handles the result of testing a URL. Currently just prints the result.
fn handle_test_result(path: &str, url: &str, result: TestResult) {
    print!("{} - {} - ", path.blue(), url);

    match result {
        TestResult::Ok => println!("{}", "OK".green()),
        TestResult::NotFound => println!("{}", "Not Found".red()),
        TestResult::Redirect(redirect) => println!("{}: {}", "Redirect Found".yellow(), redirect),
        TestResult::Error(e) => println!("{}: {}", "Failed".red(), e),
    }
}