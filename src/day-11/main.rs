use std::cell::RefCell;
use std::collections::HashMap;

use divrem::*;
use either::{Either, Left, Right};
use itertools::Itertools;
use nom::bytes::complete::take_while;
use nom::character::complete as cc;
use nom::combinator::{all_consuming, map_res};
use nom::error::ErrorKind;
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::{bytes::complete::tag, Finish, IResult, Parser};

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Operation {
  pub op: char,
  pub left_arg: String,
  pub right_arg: Either<String, i32>,
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq)]
pub struct Monkey {
  pub monkey_number: i32,
  pub items: Vec<i32>,
  pub operation: Operation,
  pub test_divisible_number: i32,
  pub if_true_receiver: i32,
  pub if_false_receiver: i32,
  pub total_items_inspections: i32,
}

fn main() {
  let monkeys = all_consuming(parse_all_monkeys)(include_str!("input.txt"))
    .finish()
    .unwrap()
    .1;

  let monkeys_iterator = RefCell::from(
    monkeys
      .iter()
      .sorted()
      .map(|m| (m.monkey_number, m.clone()))
      .collect::<HashMap<_, _>>(),
  );
  let mut monkeys_reference = RefCell::from(
    monkeys
      .iter()
      .sorted()
      .map(|m| (m.monkey_number, m.clone()))
      .collect::<HashMap<_, _>>(),
  );

  const NUMBER_ROUNDS: i32 = 20;
  let mut i: i32 = 0;

  while i < NUMBER_ROUNDS {
    monkeys_iterator
      .borrow_mut()
      .iter_mut()
      .sorted()
      .for_each(|(_, monkey)| {
        play(monkey, &mut monkeys_reference);
      });
    i = i + 1;
  }

  let top_results = monkeys_reference
    .get_mut()
    .iter()
    .sorted_by(|a, b| {
      b.1
        .total_items_inspections
        .cmp(&a.1.total_items_inspections)
    })
    .collect::<Vec<_>>();

  println!(
    "Total inspections: {}",
    top_results[0].1.total_items_inspections * top_results[1].1.total_items_inspections
  );
}

fn play(monkey: &mut Monkey, monkeys: &mut RefCell<HashMap<i32, Monkey>>) {
  let monkey_items = monkeys
    .get_mut()
    .get(&monkey.monkey_number)
    .unwrap()
    .clone()
    .items;
  for item in &monkey_items {
    let result = match monkey.operation.op {
      '+' => match monkey.operation.right_arg {
        Left(_) => item + item,
        Right(right_arg) => item + right_arg,
      },
      '*' => match monkey.operation.right_arg {
        Left(_) => item * item,
        Right(right_arg) => item * right_arg,
      },
      _ => panic!("Unknown operation"),
    };

    match result.div_rem(3) {
      (div, _) => match div % monkey.test_divisible_number {
        0 => {
          let mut target_monkey = monkeys
            .get_mut()
            .get(&monkey.if_true_receiver)
            .unwrap()
            .clone();
          target_monkey.items.push(div);
          monkeys
            .get_mut()
            .insert(monkey.if_true_receiver, target_monkey);
        }
        _ => {
          let mut target_monkey = monkeys
            .get_mut()
            .get(&monkey.if_false_receiver)
            .unwrap()
            .clone();
          target_monkey.items.push(div);
          monkeys
            .get_mut()
            .insert(monkey.if_false_receiver, target_monkey);
        }
      },
    }
  }
  let mut binding = monkeys.borrow_mut();
  let m = binding.get_mut(&monkey.monkey_number).unwrap();
  m.total_items_inspections = m.total_items_inspections + monkey_items.len() as i32;
  m.items = vec![];
}

pub fn parse_all_monkeys(i: &str) -> IResult<&str, Vec<Monkey>> {
  let monkey_list = separated_list1(cc::newline, parse_monkey)(i);
  return monkey_list;
}

fn parse_monkey(i: &str) -> IResult<&str, Monkey> {
  // Sample input:
  // Monkey 0:
  //   Starting items: 79, 98
  //   Operation: new = old * 19
  //   Test: divisible by 23
  //     If true: throw to monkey 2
  //     If false: throw to monkey 3

  let (i, (_, monkey_number, _, _)) = tuple((tag("Monkey "), cc::u64, tag(":"), cc::newline))(i)?;
  let (i, (_, _, items, _)) = tuple((
    cc::space1,
    tag("Starting items: "),
    separated_list1(tag(", "), cc::u64),
    cc::newline,
  ))(i)?;
  let (i, (_, _, operation, _)) = tuple((
    cc::space1,
    tag("Operation: "),
    map_res(
      tuple((
        take_while(|c| c != ' '),
        tag::<&str, &str, _>(" = "),
        take_while(|c| c != ' '),
        cc::space1,
        cc::one_of("+-*/"),
        cc::space1,
        cc::alphanumeric1.or(cc::digit1),
      )),
      |(_, _, left_arg, _, op, _, right_arg)| match right_arg.parse::<i32>().is_ok() {
        true => Ok::<Operation, ErrorKind>(Operation {
          op,
          left_arg: left_arg.to_string(),
          right_arg: Right(right_arg.parse::<i32>().unwrap()),
        }),
        false => Ok::<Operation, ErrorKind>(Operation {
          op,
          left_arg: left_arg.to_string(),
          right_arg: Left(right_arg.to_string()),
        }),
      },
    ),
    cc::newline,
  ))(i)?;
  let (i, (_, _, test, _)) = tuple((
    cc::space1,
    tag("Test: divisible by "),
    take_while(|c: char| c.is_alphanumeric()),
    cc::newline,
  ))(i)?;
  let (i, (_, _, if_true_receiver, _)) = tuple((
    cc::space1,
    tag("If true: throw to monkey "),
    take_while(|c: char| c.is_alphanumeric()),
    cc::newline,
  ))(i)?;
  let (i, (_, _, if_false_receiver, _)) = tuple((
    cc::space1,
    tag("If false: throw to monkey "),
    take_while(|c: char| c.is_alphanumeric()),
    cc::newline,
  ))(i)?;

  Ok((
    i,
    Monkey {
      monkey_number: monkey_number.to_string().parse::<i32>().unwrap(),
      items: items
        .iter()
        .map(|x| x.to_string().parse::<i32>().unwrap())
        .collect(),
      operation,
      test_divisible_number: test.to_string().parse::<i32>().unwrap(),
      if_true_receiver: if_true_receiver.to_string().parse::<i32>().unwrap(),
      if_false_receiver: if_false_receiver.to_string().parse::<i32>().unwrap(),
      total_items_inspections: 0,
    },
  ))
}
