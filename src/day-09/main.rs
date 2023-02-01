use std::env;
use std::fs;
use std::io::Result;
use std::path::PathBuf;

use linked_hash_set::LinkedHashSet;

enum Direction {
  Up,
  Down,
  Left,
  Right,
}

#[derive(Eq, Hash, PartialEq, Debug, Copy, Clone)]
struct GridPosition {
  col: i64,
  row: i64,
}

fn main() {
  let working_dir = get_current_working_dir();
  let file_path = format!("{}/src/day-09/input.txt", working_dir.unwrap().display());
  let contents = fs::read_to_string(file_path).expect("Unable to read file");

  let mut grid_positions_tail: LinkedHashSet<GridPosition> = LinkedHashSet::new();
  grid_positions_tail.insert(GridPosition { col: 0, row: 0 });
  let mut position_tail = GridPosition { col: 0, row: 0 };
  let mut position_head = GridPosition { col: 0, row: 0 };

  contents.lines().for_each(|line| {
    let direction = match line.split(" ").collect::<Vec<&str>>()[0] {
      "U" => Direction::Up,
      "D" => Direction::Down,
      "L" => Direction::Left,
      "R" => Direction::Right,
      _ => panic!("Invalid direction"),
    };
    let distance = line.split(" ").collect::<Vec<&str>>()[1]
      .parse::<i64>()
      .unwrap();

    for _ in 0..(distance) {
      move_head(&mut position_head, &direction);
      maybe_move_tail(
        &mut grid_positions_tail,
        &mut position_tail,
        position_head,
        &direction,
      );
    }
  });
  println!("Visited positions tail: {}", grid_positions_tail.len());
}

fn maybe_move_tail(
  grid_positions_tail: &mut LinkedHashSet<GridPosition>,
  position_tail: &mut GridPosition,
  position_head: GridPosition,
  direction: &Direction,
) {
  match direction {
    Direction::Up => {
      // Up
      if position_tail.col == position_head.col && (position_tail.row - position_head.row).abs() > 1
      {
        *position_tail = GridPosition {
          row: position_tail.row + 1,
          col: position_tail.col,
        };
        grid_positions_tail.insert(*position_tail);
      }
      // Up and right
      else if position_head.col > position_tail.col
        && (position_tail.row - position_head.row).abs() > 1
      {
        *position_tail = GridPosition {
          row: position_tail.row + 1,
          col: position_tail.col + 1,
        };
        grid_positions_tail.insert(*position_tail);
      }
      // Up and left
      else if position_head.col < position_tail.col
        && (position_tail.row - position_head.row).abs() > 1
      {
        *position_tail = GridPosition {
          row: position_tail.row + 1,
          col: position_tail.col - 1,
        };
        grid_positions_tail.insert(*position_tail);
      }
    }
    Direction::Down => {
      // Down
      if position_tail.col == position_head.col && (position_tail.row - position_head.row).abs() > 1
      {
        *position_tail = GridPosition {
          row: position_tail.row - 1,
          col: position_tail.col,
        };
        grid_positions_tail.insert(*position_tail);
      }
      // Down and right
      else if position_head.col > position_tail.col
        && (position_tail.row - position_head.row).abs() > 1
      {
        *position_tail = GridPosition {
          row: position_tail.row - 1,
          col: position_tail.col + 1,
        };
        grid_positions_tail.insert(*position_tail);
      }
      // Down and left
      else if position_head.col < position_tail.col
        && (position_tail.row - position_head.row).abs() > 1
      {
        *position_tail = GridPosition {
          row: position_tail.row - 1,
          col: position_tail.col - 1,
        };
        grid_positions_tail.insert(*position_tail);
      }
    }
    Direction::Right => {
      // Right
      if position_tail.row == position_head.row && (position_tail.col - position_head.col).abs() > 1
      {
        *position_tail = GridPosition {
          row: position_tail.row,
          col: position_tail.col + 1,
        };
        grid_positions_tail.insert(*position_tail);
      }
      // Right and down
      else if position_head.row < position_tail.row
        && (position_tail.col - position_head.col).abs() > 1
      {
        *position_tail = GridPosition {
          row: position_tail.row - 1,
          col: position_tail.col + 1,
        };
        grid_positions_tail.insert(*position_tail);
      }
      // Right and up
      else if position_head.row > position_tail.row
        && (position_tail.col - position_head.col).abs() > 1
      {
        *position_tail = GridPosition {
          row: position_tail.row + 1,
          col: position_tail.col + 1,
        };
        grid_positions_tail.insert(*position_tail);
      }
    }
    Direction::Left => {
      // Left
      if position_tail.row == position_head.row && (position_tail.col - position_head.col).abs() > 1
      {
        *position_tail = GridPosition {
          row: position_tail.row,
          col: position_tail.col - 1,
        };
        grid_positions_tail.insert(*position_tail);
      }
      // Left and down
      else if position_head.row < position_tail.row
        && (position_tail.col - position_head.col).abs() > 1
      {
        *position_tail = GridPosition {
          row: position_tail.row - 1,
          col: position_tail.col - 1,
        };
        grid_positions_tail.insert(*position_tail);
      }
      // Left and up
      else if position_head.row > position_tail.row
        && (position_tail.col - position_head.col).abs() > 1
      {
        *position_tail = GridPosition {
          row: position_tail.row + 1,
          col: position_tail.col - 1,
        };
        grid_positions_tail.insert(*position_tail);
      }
    }
  }
}

fn move_head(position_head: &mut GridPosition, direction: &Direction) {
  match direction {
    Direction::Up => {
      *position_head = GridPosition {
        row: position_head.row + 1,
        col: position_head.col,
      };
    }
    Direction::Down => {
      *position_head = GridPosition {
        row: position_head.row - 1,
        col: position_head.col,
      };
    }
    Direction::Left => {
      *position_head = GridPosition {
        row: position_head.row,
        col: position_head.col - 1,
      };
    }
    Direction::Right => {
      *position_head = GridPosition {
        row: position_head.row,
        col: position_head.col + 1,
      };
    }
  };
}

fn get_current_working_dir() -> Result<PathBuf> {
  env::current_dir()
}
