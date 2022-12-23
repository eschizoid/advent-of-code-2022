use std::env;
use std::fs;
use std::io::Result;
use std::path::PathBuf;

fn get_current_working_dir() -> Result<PathBuf> {
    env::current_dir()
}

fn unique_chars(s: &str) -> bool {
    let mut chars = s.chars().collect::<Vec<char>>();
    chars.sort();
    chars.dedup();
    return if chars.len() == 4 { true } else { false };
}

fn main() {
    let working_dir = get_current_working_dir();
    let file_path = format!("{}/src/day-6/input.txt", working_dir.unwrap().display());
    let contents = fs::read_to_string(file_path).expect("Unable to read file");

    let all_chars = contents.to_string();
    let mut i = 0;
    while i <= all_chars.len() {
        let marker = format!(
            "{}{}{}{}",
            all_chars.chars().nth(i).unwrap(),
            all_chars.chars().nth(i + 1).unwrap(),
            all_chars.chars().nth(i + 2).unwrap(),
            all_chars.chars().nth(i + 3).unwrap()
        );
        match unique_chars(&marker) {
            true => {
                println!("{}", i + 4);
                i = i + 1;
                break;
            }
            false => {
                i = i + 1;
                continue;
            }
        }
    }
}
