use std::env;
use std::fs;
use std::path::PathBuf;

fn get_current_working_dir() -> std::io::Result<PathBuf> {
  env::current_dir()
}

fn main() {
  let working_dir = get_current_working_dir();
  let file_path = format!("{}/src/day-01/input.txt", working_dir.unwrap().display());
  let mut calories = vec![];
  let mut calories_per_elf = vec![];

  let contents = fs::read_to_string(file_path).expect("Unable to read file");

  contents.lines().for_each(|line| {
    if line.len() != 0 {
      calories.push(line.parse::<i32>().unwrap());
    } else {
      calories_per_elf.push(calories.iter().sum::<i32>());
      calories.clear()
    }
  });

  let max = calories_per_elf.iter().max().unwrap();
  print!("{}", max);
}
