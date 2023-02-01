use std::env;
use std::fs;
use std::io::Result;
use std::path::PathBuf;

fn get_current_working_dir() -> Result<PathBuf> {
  env::current_dir()
}

fn main() {
  const WIN_POINTS: i32 = 6;
  const DRAW_POINTS: i32 = 3;
  const LOSS_POINTS: i32 = 0;

  const ROCK_POINTS: i32 = 1;
  const PAPER_POINTS: i32 = 2;
  const SCISSORS_POINTS: i32 = 3;

  const OPPONENT_ROCK: &str = "A";
  const OPPONENT_PAPER: &str = "B";
  const OPPONENT_SCISSORS: &str = "C";

  const ME_ROCK: &str = "X";
  const ME_PAPER: &str = "Y";
  const ME_SCISSORS: &str = "Z";

  let working_dir = get_current_working_dir();
  let file_path = format!("{}/src/day-02/input.txt", working_dir.unwrap().display());
  let contents = fs::read_to_string(file_path).expect("Unable to read file");
  let mut result = vec![];

  contents.lines().for_each(|line| {
    let team = line.split(" ").collect::<Vec<&str>>();
    let opponent = team[0];
    let me = team[1];
    match opponent {
      OPPONENT_ROCK => match me {
        ME_ROCK => result.push(DRAW_POINTS + ROCK_POINTS),
        ME_PAPER => result.push(WIN_POINTS + PAPER_POINTS),
        ME_SCISSORS => result.push(LOSS_POINTS + SCISSORS_POINTS),
        _ => {}
      },
      OPPONENT_PAPER => match me {
        ME_ROCK => result.push(LOSS_POINTS + ROCK_POINTS),
        ME_PAPER => result.push(DRAW_POINTS + PAPER_POINTS),
        ME_SCISSORS => result.push(WIN_POINTS + SCISSORS_POINTS),
        _ => {}
      },
      OPPONENT_SCISSORS => match me {
        ME_ROCK => result.push(WIN_POINTS + ROCK_POINTS),
        ME_PAPER => result.push(LOSS_POINTS + PAPER_POINTS),
        ME_SCISSORS => result.push(DRAW_POINTS + SCISSORS_POINTS),
        _ => {}
      },
      _ => {}
    }
  });
  let result: i32 = result.iter().sum();
  print!("{}", result);
}
