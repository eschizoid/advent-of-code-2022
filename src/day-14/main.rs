use itertools::Itertools;
use pathfinding::matrix::Matrix;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Element {
  Rock,
  Air,
  Sand,
  Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Cell {
  y: usize,
  x: usize,
}

impl Cell {
  fn move_down(&self) -> Self {
    Cell {
      x: self.x,
      y: self.y + 1,
    }
  }

  fn move_right_diagonal(&self) -> Self {
    Cell {
      x: self.x + 1,
      y: self.y + 1,
    }
  }

  fn move_left_diagonal(&self) -> Self {
    Cell {
      x: self.x - 1,
      y: self.y + 1,
    }
  }

  fn as_tuple(&self) -> (usize, usize) {
    (self.y as usize, self.x as usize)
  }
}

fn main() {
  let input = include_str!("input.txt").split("\n").collect::<Vec<&str>>();
  let rocks = input.iter().collect_vec();
  let mut rock_paths = get_rock_paths(rocks).clone();
  let initial_matrix = get_initial_matrix(&mut rock_paths);
  let matrix_with_rocks = add_rocks_to_matrix(initial_matrix, rock_paths);
  drop_sand_to_matrix(matrix_with_rocks);
}

fn drop_sand_to_matrix(mut matrix_with_rocks: Matrix<Element>) {
  //   4 4 4 4 4 4 5 5 5 0
  //   9 9 9 9 9 9 0 0 0 0
  //   4 5 6 7 8 9 0 1 2 3
  // 0 . . . . . . + . . .
  // 1 . . . . . . . . . .
  // 2 . . . . . . o . . .
  // 3 . . . . . o o o . .
  // 4 . . . . # o o o # #
  // 5 . . . o # o o o # .
  // 6 . . # # # o o o # .
  // 7 . . . . o o o o # .
  // 8 . o . o o o o o # .
  // 9 # # # # # # # # # .

  let mut total_sand_units = 0;
  let mut current_position = Cell { y: 0, x: 500 };

  loop {
    let left_position = current_position.move_left_diagonal().as_tuple();
    let down_position = current_position.move_down().as_tuple();
    let right_position = current_position.move_right_diagonal().as_tuple();

    let left_element = *matrix_with_rocks
      .get_mut(left_position)
      .unwrap_or(&mut Element::Unknown);
    let down_element = *matrix_with_rocks
      .get_mut(down_position)
      .unwrap_or(&mut Element::Unknown);
    let right_element = *matrix_with_rocks
      .get_mut(right_position)
      .unwrap_or(&mut Element::Unknown);

    if down_element == Element::Unknown
      || right_element == Element::Unknown
      || left_element == Element::Unknown
    {
      println!("Total sand units: {:?}", total_sand_units);
      break;
    }

    match down_element {
      Element::Air => {
        current_position = Cell {
          x: current_position.x,
          y: current_position.y + 1,
        };
      }
      Element::Rock | Element::Sand
        if ((left_element == Element::Rock && right_element == Element::Rock)
          || (left_element == Element::Sand && right_element == Element::Sand)
          || (left_element == Element::Rock && right_element == Element::Sand)
          || (left_element == Element::Sand && right_element == Element::Rock)) =>
      {
        *matrix_with_rocks
          .get_mut(current_position.as_tuple())
          .unwrap() = Element::Sand;
        total_sand_units += 1;
        current_position = Cell { y: 0, x: 500 };
      }
      Element::Rock | Element::Sand if (left_element == Element::Air) => {
        current_position = Cell {
          x: current_position.x - 1,
          y: current_position.y + 1,
        }
      }
      Element::Rock | Element::Sand if (right_element == Element::Air) => {
        current_position = Cell {
          x: current_position.x + 1,
          y: current_position.y + 1,
        }
      }
      _ => {
        panic!("Something went wrong")
      }
    }
  }
  print_matrix(matrix_with_rocks);
}

fn add_rocks_to_matrix(
  initial_matrix: Matrix<Element>,
  rock_paths: Vec<Vec<Cell>>,
) -> Matrix<Element> {
  let mut rock_air_matrix = initial_matrix.clone();
  for rock_path in rock_paths.iter().collect_vec() {
    for (cell, next_cell) in rock_path.iter().zip(rock_path.iter().skip(1)) {
      match cell.x < next_cell.x && cell.y == next_cell.y {
        true => {
          let mut x = next_cell.x;
          while cell.x <= x {
            let element = rock_air_matrix
              .get_mut((cell.y as usize, x as usize))
              .unwrap();
            *element = Element::Rock;
            x -= 1;
          }
        }
        false => match next_cell.x < cell.x && cell.y == next_cell.y {
          true => {
            let mut x = cell.x;
            while next_cell.x <= x {
              let element = rock_air_matrix
                .get_mut((cell.y as usize, x as usize))
                .unwrap();
              *element = Element::Rock;
              x -= 1;
            }
          }
          false => match next_cell.y < cell.y && cell.x == next_cell.x {
            true => {
              let mut y = cell.y;
              while next_cell.y <= y {
                let element = rock_air_matrix
                  .get_mut((y as usize, cell.x as usize))
                  .unwrap();
                *element = Element::Rock;
                y -= 1;
              }
            }
            false => match cell.y < next_cell.y && cell.x == next_cell.x {
              true => {
                let mut y = next_cell.y;
                while cell.y <= y {
                  let element = rock_air_matrix
                    .get_mut((y as usize, cell.x as usize))
                    .unwrap();
                  *element = Element::Rock;
                  y -= 1;
                }
              }
              false => {
                panic!("Something went wrong")
              }
            },
          },
        },
      }
    }
  }
  rock_air_matrix
}

fn get_rock_paths(rocks: Vec<&&str>) -> Vec<Vec<Cell>> {
  let rock_paths = rocks
    .iter()
    .map(|coor| String::from(coor.to_string()))
    .into_iter()
    .filter(|coor| coor != "")
    .map(|coor| {
      coor
        .split(" -> ")
        .into_iter()
        .collect_vec()
        .into_iter()
        .map(|c| c.split(",").collect_vec())
        .collect_vec()
        .into_iter()
        .map(|val| Cell {
          x: val[0].parse::<usize>().unwrap(),
          y: val[1].parse::<usize>().unwrap(),
        })
        .collect_vec()
    })
    .collect_vec();
  rock_paths
}

fn get_initial_matrix(rock_paths: &mut Vec<Vec<Cell>>) -> Matrix<Element> {
  let max_x = rock_paths
    .iter()
    .map(|path| path.iter().map(|c| c.x).max().unwrap())
    .max()
    .unwrap()
    + 1;
  let max_y = rock_paths
    .iter()
    .map(|path| path.iter().map(|c| c.y).max().unwrap())
    .max()
    .unwrap()
    + 1;

  let mut air = Vec::new();
  for _ in 0..max_y {
    let mut row = Vec::new();
    for _ in 0..max_x {
      row.push(Element::Air);
    }
    air.push(row);
  }
  Matrix::from_vec(max_y, max_x, air.into_iter().flatten().collect()).unwrap()
}

fn print_matrix(matrix_with_rocks: Matrix<Element>) {
  println!("Rows: {}", matrix_with_rocks.rows);
  println!("Columns: {}", matrix_with_rocks.columns);

  let v = matrix_with_rocks.to_vec();
  v.iter().enumerate().for_each(|val| {
    if val.0 % matrix_with_rocks.columns == 0 {
      println!();
    } else {
      match val.1 {
        Element::Air => print!("."),
        Element::Rock => print!("#"),
        Element::Sand => print!("o"),
        _ => print!(" "),
      }
    }
  });
}
