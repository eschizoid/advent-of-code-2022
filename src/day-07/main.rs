use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Result;
use std::path::PathBuf;

use indextree::{Arena, NodeEdge, NodeId};

fn main() {
  let working_dir = get_current_working_dir();
  let file_path = format!("{}/src/day-07/input.txt", working_dir.unwrap().display());
  let contents = fs::read_to_string(file_path).expect("Unable to read file");

  let mut arena: Arena<String> = Arena::new();
  let root_node = arena.new_node(String::from("/"));
  let mut current_node = root_node;
  let mut nodes: HashMap<String, NodeId> = HashMap::new();

  contents.lines().enumerate().for_each(|line_command| {
    parse_input(line_command, &mut arena, &mut current_node, &mut nodes);
  });

  println!("{}", root_node.debug_pretty_print(&arena));

  traverse_tree(&mut arena, root_node);
}

fn traverse_tree(arena: &mut Arena<String>, root_node: NodeId) {
  let mut iterator = root_node.traverse(&arena).peekable();
  let mut sizes: HashMap<NodeId, i64> = HashMap::new();
  while let Some(_) = iterator.peek() {
    let node = iterator.next().unwrap();
    match node {
      NodeEdge::End(node) => {
        let node_parent = &arena[node].parent();
        match node_parent {
          Some(node_parent) => match &arena[node].get().parse::<i64>() {
            Ok(size) => match sizes.contains_key(node_parent) {
              true => {
                let size = sizes.get(node_parent).unwrap() + size;
                sizes.insert(*node_parent, size);
              }
              false => {
                sizes.insert(*node_parent, *size);
              }
            },
            _ => {
              let child_size = sizes.get(&node);
              match child_size {
                Some(child_size) => match sizes.contains_key(node_parent) {
                  true => {
                    let size = sizes.get(node_parent).unwrap() + child_size;
                    sizes.insert(*node_parent, size);
                  }
                  false => {
                    sizes.insert(*node_parent, *child_size);
                  }
                },
                _ => {}
              }
            }
          },
          _ => {}
        }
      }
      _ => {}
    }
  }
  println!(
    "total {}",
    sizes
      .into_values()
      .filter(|size| *size <= 100_000)
      .sum::<i64>()
  );
}

fn parse_input(
  line_command: (usize, &str),
  arena: &mut Arena<String>,
  current_node: &mut NodeId,
  nodes: &mut HashMap<String, NodeId>,
) {
  let working_dir = get_current_working_dir();
  let file_path = format!("{}/src/day-07/input.txt", working_dir.unwrap().display());

  match line_command.1.starts_with("$ ls") {
    true => {
      let (line_number, _) = line_command;
      let ls_iterator = fs::read_to_string(file_path).expect("Unable to read file");

      let ls_results = ls_iterator
        .lines()
        .enumerate()
        .filter(|line| line.0 > line_number)
        .take_while(|line| line.1.starts_with("$") != true)
        .map(|line| line.1)
        .collect::<Vec<&str>>();

      ls_results.iter().for_each(|ls_result| match ls_result {
        result if result.starts_with("dir") => {
          let dir_name = result.split(" ").collect::<Vec<&str>>()[1];
          let dir_node = arena.new_node(String::from(dir_name));
          current_node.append(dir_node, arena);
          nodes.insert(String::from(dir_name), dir_node);
        }
        _ => {
          let file_size = ls_result.split(" ").collect::<Vec<&str>>()[0];
          let file_node = arena.new_node(String::from(file_size));
          current_node.append(file_node, arena);
          nodes.insert(String::from(file_size.to_string()), file_node);
        }
      });
    }
    _ => {}
  }

  match line_command.1.starts_with("$ cd") {
    true if line_command.1.contains("..") => {
      let parent_node = current_node.ancestors(arena).into_iter().nth(0).unwrap();
      *current_node = parent_node;
    }
    true if line_command.1.contains("/") == false => {
      let dir_name = line_command.1.split_whitespace().nth(2).unwrap();
      let new_node = *nodes.get(dir_name).unwrap();
      *current_node = new_node;
    }
    _ => {}
  }
}

fn get_current_working_dir() -> Result<PathBuf> {
  env::current_dir()
}
