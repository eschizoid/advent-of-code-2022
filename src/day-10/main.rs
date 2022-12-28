use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Result;
use std::path::PathBuf;

use nom::{
  branch::alt,
  bytes::complete::tag,
  combinator::{map, value},
  sequence::preceded,
  IResult,
};

#[derive(Debug, Clone)]
enum Instruction {
  Noop,
  Addx(i32),
}

fn main() {
  let working_dir = get_current_working_dir();
  let file_path = format!("{}/src/day-10/input.txt", working_dir.unwrap().display());
  let contents = fs::read_to_string(file_path).expect("Unable to read file");

  let mut cycles = 0;
  let mut register_x = 1;
  let mut results: HashMap<i32, i32> = HashMap::new();

  contents.lines().for_each(|line| {
    let instruction: IResult<&str, Instruction> = parse_instruction(line);
    match instruction {
      Ok((_, instruction)) => match instruction {
        Instruction::Noop => {
          cycles = cycles + 1;
          results.insert(cycles, register_x);
        }
        Instruction::Addx(x) => {
          let cycle_plus_one = cycles + 1;
          let cycle_plus_two = cycles + 2;
          results.insert(cycle_plus_one, register_x);
          results.insert(cycle_plus_two, register_x);
          register_x = register_x + x;
          cycles = cycle_plus_two;
          results.insert(cycles, register_x);
        }
      },
      Err(_) => panic!("Invalid instruction: {:?}", instruction),
    }
  });

  let result_20 = 20 * results.get(&(20 - 1)).unwrap();
  let result_60 = 60 * results.get(&(60 - 1)).unwrap();
  let result_100 = 100 * results.get(&(100 - 1)).unwrap();
  let result_140 = 140 * results.get(&(140 - 1)).unwrap();
  let result_180 = 180 * results.get(&(180 - 1)).unwrap();
  let result_220 = 220 * results.get(&(220 - 1)).unwrap();

  println!(
    "Total: {}",
    result_20 + result_60 + result_100 + result_140 + result_180 + result_220
  );
}

fn parse_instruction(i: &str) -> IResult<&str, Instruction> {
  let noop = tag("noop");
  let addx = preceded(tag("addx "), nom::character::complete::i32);
  alt((value(Instruction::Noop, noop), map(addx, Instruction::Addx)))(i)
}

fn get_current_working_dir() -> Result<PathBuf> {
  env::current_dir()
}
