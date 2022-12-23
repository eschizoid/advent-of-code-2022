use std::collections::HashMap;
use std::collections::HashSet;
use std::env;
use std::fs;
use std::io::Result;
use std::path::PathBuf;

fn get_current_working_dir() -> Result<PathBuf> {
    env::current_dir()
}

fn main() {
    let working_dir = get_current_working_dir();
    let file_path = format!("{}/src/day-3/input.txt", working_dir.unwrap().display());
    let contents = fs::read_to_string(file_path).expect("Unable to read file");

    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .enumerate()
        .map(|(i, c)| (c, i + 1))
        .collect::<HashMap<char, usize>>();

    let mut total = vec![];

    contents.lines().for_each(|line| {
        let (split1, split2) = line.split_at(line.len() / 2);
        let set1 = split1.chars().collect::<HashSet<_>>();
        let set2 = split2.chars().collect::<HashSet<_>>();
        let c = set1.intersection(&set2).map(|i| *i).collect::<Vec<_>>();
        total.push(alphabet.get(&c[0]).unwrap());
    });
    println!("{}", total.iter().copied().sum::<usize>());
}
