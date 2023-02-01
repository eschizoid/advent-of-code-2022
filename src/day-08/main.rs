use std::fs;
use std::io::Result;
use std::path::PathBuf;
use std::{env, usize};

use grid::Grid;
use grid::*;

fn main() {
  let working_dir = get_current_working_dir();
  let file_path = format!("{}/src/day-08/input.txt", working_dir.unwrap().display());
  let contents = fs::read_to_string(file_path).expect("Unable to read file");

  const COLS: usize = 99;
  const ROWS: usize = 99;

  const EDGE_COLS_TREES_VISIBLE: usize = (COLS * 2) - 2;
  const EDGE_ROW_TREES_VISIBLE: usize = (ROWS * 2) - 2;

  let mut grid: Grid<i64> = grid![];
  contents.lines().enumerate().for_each(|line| {
    create_grid(line.1, line.0, &mut grid);
  });

  println!("{:#?}", grid);

  let mut counter = 0;
  for col in 1..COLS - 1 {
    for row in 1..ROWS - 1 {
      let value = grid[row][col];
      let tree_house_visible = is_treehouse_visible(
        row.try_into().unwrap(),
        col.try_into().unwrap(),
        &mut grid,
        value.try_into().unwrap(),
      );
      if tree_house_visible {
        counter = counter + 1;
      }
    }
  }
  println!(
    "visible trees: {}",
    counter + EDGE_COLS_TREES_VISIBLE + EDGE_ROW_TREES_VISIBLE
  );
}

fn is_treehouse_visible(row: i64, col: i64, grid: &mut Grid<i64>, value: i64) -> bool {
  let (right_row_values, left_row_values) = iterate_rows(row, col, grid);
  let (up_col_values, down_col_values) = iterate_columns(row, col, grid);
  let is_visible_right = right_row_values.iter().all(|&x| x < value);
  let is_visible_left = left_row_values.iter().all(|&x| x < value);
  let is_visible_up = up_col_values.iter().all(|&x| x < value);
  let is_visible_down = down_col_values.iter().all(|&x| x < value);
  return is_visible_right || is_visible_left || is_visible_up || is_visible_down;
}

fn iterate_columns(row: i64, col: i64, grid: &mut Grid<i64>) -> (Vec<i64>, Vec<i64>) {
  let row_iter = grid
    .iter_col(col.try_into().unwrap())
    .map(|x| *x)
    .collect::<Vec<i64>>();

  let up_col_values = row_iter
    .iter()
    .enumerate()
    .filter(|x| x.0 < row.try_into().unwrap())
    .map(|x| *x.1)
    .collect::<Vec<i64>>();

  let down_col_values = row_iter
    .iter()
    .enumerate()
    .filter(|x| x.0 > row.try_into().unwrap())
    .map(|x| *x.1)
    .collect::<Vec<i64>>();

  return (up_col_values, down_col_values);
}

fn iterate_rows(row: i64, col: i64, grid: &mut Grid<i64>) -> (Vec<i64>, Vec<i64>) {
  let row_iter = grid
    .iter_row(row.try_into().unwrap())
    .map(|x| *x)
    .collect::<Vec<i64>>();

  let right_row_values = row_iter
    .iter()
    .enumerate()
    .filter(|x| x.0 > col.try_into().unwrap())
    .map(|x| *x.1)
    .collect::<Vec<i64>>();

  let left_row_values = row_iter
    .iter()
    .enumerate()
    .filter(|x| x.0 < col.try_into().unwrap())
    .map(|x| *x.1)
    .collect::<Vec<i64>>();

  return (right_row_values, left_row_values);
}

fn create_grid(line: &str, i: usize, grid: &mut Grid<i64>) {
  let row = line
    .chars()
    .map(|c| c.to_digit(10).unwrap().try_into().unwrap())
    .collect::<Vec<i64>>();
  grid.insert_row(i, row);
}

fn get_current_working_dir() -> Result<PathBuf> {
  env::current_dir()
}
