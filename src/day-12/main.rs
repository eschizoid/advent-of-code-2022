use std::collections::HashMap;
use std::string::ToString;

use itertools::Itertools;
use pathfinding::matrix::Matrix;
use pathfinding::prelude::dijkstra;

fn main() {
  let start: Cell = Cell { x: 0, y: 20 };
  let end: Cell = Cell { x: 77, y: 20 };
  let input = include_str!("input.txt").split("\n").collect::<Vec<&str>>();

  let alphabet_map = build_alphabet_map();
  let matrix = build_matrix(input, alphabet_map);
  let result = dijkstra(&start, |p| p.successors(&matrix), |p| *p == end);
  let result = result.unwrap();

  println!("Path: {:?}", result.0);
  println!("Number of moves: {:?}", result.1);
}

fn build_alphabet_map() -> HashMap<String, u32> {
  let mut alphabet_map: HashMap<String, u32> = HashMap::from_iter(
    (b'a'..=b'z')
      .enumerate()
      .filter(|c| (c.1 as char).is_alphabetic())
      .map(|c| (String::from(c.1 as char), (c.0 as u32 + 1)))
      .collect::<Vec<(String, u32)>>(),
  );
  alphabet_map.insert(String::from("S"), 1);
  alphabet_map.insert(String::from("E"), 27);
  alphabet_map
}

fn build_matrix(lines: Vec<&str>, alphabet_numeric_values: HashMap<String, u32>) -> Matrix<u32> {
  let rows: Vec<Vec<u32>> = lines
    .iter()
    .map(|line| {
      line
        .chars()
        .map(|c| match c {
          c if c.is_alphabetic() => alphabet_numeric_values[&c.to_string()],
          _ => {
            panic!("Unknown char: {}", c)
          }
        })
        .collect_vec()
    })
    .collect_vec();

  return Matrix::from_vec(
    rows.len() - 1,
    rows[0].len(),
    rows.into_iter().flatten().collect(),
  )
  .unwrap();
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Cell {
  y: usize,
  x: usize,
}

impl Cell {
  fn successors(&self, matrix: &Matrix<u32>) -> Vec<(Cell, i32)> {
    let current_value = matrix[(self.y as usize, self.x as usize)] as i32;
    return matrix
      .neighbours((self.y as usize, self.x as usize), false)
      .collect_vec()
      .iter()
      .filter(|neighbor: &&(usize, usize)| {
        current_value >= matrix[**neighbor] as i32
          || current_value.abs_diff(matrix[**neighbor] as i32) <= 1
      })
      .collect_vec()
      .iter()
      .map(|cell| {
        (
          Cell {
            y: cell.0,
            x: cell.1,
          },
          1,
        )
      })
      .collect_vec();
  }
}
