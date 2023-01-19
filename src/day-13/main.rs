use itertools::EitherOrBoth::{Both, Left, Right};
use itertools::Itertools;
use serde_json::Value;

#[derive(Debug)]
struct Pair {
  pub left: Vec<Value>,
  pub right: Vec<Value>,
}

fn main() {
  let input = include_str!("input.txt").split("\n").collect::<Vec<&str>>();
  let pairs = input
    .iter()
    .map(|line| line.to_string())
    .filter(|line| line.len() > 0)
    .tuples()
    .map(|(left, right)| Pair {
      left: Value::Array(serde_json::from_str(left.as_str()).unwrap())
        .as_array()
        .unwrap()
        .to_vec(),
      right: Value::Array(serde_json::from_str(right.as_str()).unwrap())
        .as_array()
        .unwrap()
        .to_vec(),
    })
    .collect::<Vec<Pair>>();

  let mut sum_pair_index: Vec<i32> = Vec::new();
  for index_pair in pairs.iter().enumerate().map(|(i, pair)| (i as i32, pair)) {
    println!("Processing pair [{}]: {:?}", index_pair.0 + 1, index_pair.1);
    parse_json(index_pair, &mut sum_pair_index);
  }
  sum_pair_index.dedup();
  println!("Ordered pairs: {:?}", sum_pair_index);
  println!("Ordered pairs len: {}", sum_pair_index.len());
  println!("Sum pair index: {}", sum_pair_index.iter().sum::<i32>());
}

fn parse_json(index_pair: (i32, &Pair), sum_pair_index: &mut Vec<i32>) {
  let mut left_array = index_pair.1.left.iter();
  let mut right_array = index_pair.1.right.iter();
  let mut pair_iterator = left_array.zip_longest(right_array).clone().peekable();

  'pairs: while {
    let mut item_iterator = pair_iterator.clone();
    let mut pair = pair_iterator.next();
    'items: while {
      match pair {
        Some(Both(left, right)) => match left.is_array() && right.is_array() {
          true => {
            left_array = left.as_array().unwrap().iter();
            right_array = right.as_array().unwrap().iter();
            item_iterator = left_array.zip_longest(right_array).clone().peekable();
            pair = item_iterator.next();
            continue 'items;
          }
          false => match left.is_array() && right.is_i64() {
            true => {
              let first_element_left = left.as_array().unwrap().first();
              match first_element_left {
                Some(Value::Number(n)) => {
                  if n.as_i64().unwrap() == right.as_i64().unwrap() {
                    pair = item_iterator.next();
                    continue 'items;
                  } else {
                    if n.as_i64().unwrap() < right.as_i64().unwrap() {
                      sum_pair_index.push(index_pair.0 + 1);
                    }
                    break 'pairs;
                  }
                }
                Some(Value::Array(a)) => {
                  let left_array = a.to_vec();
                  if !left_array.is_empty() {
                    let elements = left_array
                      .iter()
                      .collect::<Vec<&Value>>()
                      .iter()
                      .map(|x| x.as_i64().unwrap())
                      .collect::<Vec<i64>>();
                    if *elements.first().unwrap() == right.as_i64().unwrap() {
                      pair = item_iterator.next();
                      continue 'items;
                    } else {
                      if *elements.first().unwrap() < right.as_i64().unwrap() {
                        sum_pair_index.push(index_pair.0 + 1);
                      }
                      continue 'pairs;
                    }
                  } else {
                    sum_pair_index.push(index_pair.0 + 1);
                    break 'pairs;
                  }
                }
                _ => {
                  sum_pair_index.push(index_pair.0 + 1);
                  break 'pairs;
                }
              }
            }
            false => match left.is_i64() && right.is_array() {
              true => {
                let elements_right = right.as_array().unwrap();
                let first_element_right = elements_right.first();
                match first_element_right {
                  Some(Value::Number(n)) => {
                    if left.as_i64().unwrap() == n.as_i64().unwrap() {
                      pair = item_iterator.next();
                      continue 'items;
                    } else {
                      if left.as_i64().unwrap() < n.as_i64().unwrap() {
                        sum_pair_index.push(index_pair.0 + 1);
                      }
                      break 'pairs;
                    }
                  }
                  Some(Value::Array(a)) => {
                    let right_array = a.to_vec();
                    if !right_array.is_empty() {
                      let elements = right_array
                        .iter()
                        .collect::<Vec<&Value>>()
                        .iter()
                        .map(|x| x.as_i64().unwrap())
                        .collect::<Vec<i64>>();
                      if left.as_i64().unwrap() == *elements.first().unwrap() {
                        let initial: Vec<i64> = Vec::new();
                        let mut flatten_elements =
                          elements_right.into_iter().fold(initial, |mut acc, val| {
                            match val {
                              Value::Number(num) => {
                                acc.push(num.as_i64().unwrap());
                              }
                              Value::Array(arr) => {
                                acc.extend(arr.iter().map(|x| x.as_i64().unwrap()).collect_vec());
                              }
                              _ => panic!("invalid input"),
                            }
                            acc
                          });
                        flatten_elements.sort();
                        if *flatten_elements.last().unwrap() > left.as_i64().unwrap() {
                          sum_pair_index.push(index_pair.0 + 1);
                          break 'pairs;
                        }
                        pair = item_iterator.next();
                        continue 'items;
                      } else {
                        if left.as_i64().unwrap() < *elements.first().unwrap() {
                          sum_pair_index.push(index_pair.0 + 1);
                        }
                        break 'pairs;
                      }
                    } else {
                      break 'pairs;
                    }
                  }
                  _ => {
                    break 'pairs;
                  }
                }
              }
              false => match left.is_i64() && right.is_i64() {
                true => {
                  if left.as_i64().unwrap() == right.as_i64().unwrap() {
                    pair = item_iterator.next();
                    continue 'items;
                  } else {
                    if left.as_i64().unwrap() < right.as_i64().unwrap() {
                      sum_pair_index.push(index_pair.0 + 1);
                    }
                    break 'pairs;
                  }
                }
                false => {
                  panic!("invalid input");
                }
              },
            },
          },
        },
        Some(Left(_)) => {
          break 'pairs;
        }
        Some(Right(_)) => {
          sum_pair_index.push(index_pair.0 + 1);
          break 'pairs;
        }
        _ => {}
      }
      item_iterator.peek().is_some()
    } {}
    pair_iterator.peek().is_some()
  } {}
}
