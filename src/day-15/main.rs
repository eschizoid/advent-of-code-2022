use std::collections::HashMap;
use std::ops::RangeInclusive;

use itertools::Itertools;
use nom::bytes::complete::take_while;
use nom::character::complete as cc;
use nom::combinator::{all_consuming, map_res};
use nom::multi::separated_list1;
use nom::sequence::tuple;
use nom::{bytes::complete::tag, Finish, IResult};

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Point {
  pub x: i32,
  pub y: i32,
}

#[derive(Clone, Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Pair {
  pub sensor: Point,
  pub beacon: Point,
  pub manhattan_distance: i32,
}

fn main() {
  const INTERESTED_Y: i32 = 2_000_000;

  let pairs = all_consuming(parse_all_positions)(include_str!("input.txt"))
    .finish()
    .unwrap()
    .1;

  let mut sensor_position = pairs
    .clone()
    .into_iter()
    .filter(|p| p.sensor.y == INTERESTED_Y)
    .map(|p| p.sensor.x)
    .collect_vec();

  let beacon_position = pairs
    .clone()
    .into_iter()
    .filter(|p| p.beacon.y == INTERESTED_Y)
    .map(|p| p.beacon.x)
    .collect_vec();

  sensor_position.extend(beacon_position);
  sensor_position.sort();
  sensor_position.dedup();

  let pair_ranges = get_ranges(pairs, INTERESTED_Y);
  let mut ranges = Vec::from_iter(
    pair_ranges
      .values()
      .filter(|r| r.len() > 0)
      .map(|r| r.clone())
      .collect_vec()
      .into_iter()
      .flatten()
      .collect_vec()
      .into_iter()
      .flatten(),
  );
  ranges.sort();
  ranges.dedup();

  println!(
    "Total positions: {:?}",
    ranges.len() - sensor_position.len()
  );
}

fn get_ranges(pairs: Vec<Pair>, interested_y: i32) -> HashMap<Pair, Vec<RangeInclusive<i32>>> {
  let mut pairs_with_ranges = HashMap::new();

  for pair in &pairs {
    let mut ranges = Vec::new();
    let mut radius = pair.manhattan_distance;
    let mut current_up_y = pair.sensor.y;
    let mut current_down_y = pair.sensor.y;
    let current_x = pair.sensor.x;

    while radius >= 0 {
      if current_up_y == interested_y || current_down_y == interested_y {
        ranges.push(RangeInclusive::new(current_x - radius, current_x + radius));
      }
      current_up_y += 1;
      current_down_y -= 1;
      radius -= 1;
    }
    pairs_with_ranges.insert(pair.clone(), ranges.clone());
  }
  return pairs_with_ranges;
}

fn parse_all_positions(i: &str) -> IResult<&str, Vec<Pair>> {
  return separated_list1(cc::newline, parse_position)(i);
}

fn parse_position(i: &str) -> IResult<&str, Pair> {
  // Sensor at x=2288642, y=2282562: closest beacon is at x=1581951, y=2271709
  let (i, (_, _, sensor_x, _, sensor_y, _, _, beacon_x, _, beacon_y)) = tuple((
    tag("Sensor at "),
    tag("x="),
    map_res(
      take_while(|c: char| c.is_digit(10) || c == '-'),
      |s: &str| s.parse::<i32>(),
    ),
    tag(", y="),
    map_res(
      take_while(|c: char| c.is_digit(10) || c == '-'),
      |s: &str| s.parse::<i32>(),
    ),
    tag(": closest beacon is at "),
    tag("x="),
    map_res(
      take_while(|c: char| c.is_digit(10) || c == '-'),
      |s: &str| s.parse::<i32>(),
    ),
    tag(", y="),
    map_res(
      take_while(|c: char| c.is_digit(10) || c == '-'),
      |s: &str| s.parse::<i32>(),
    ),
  ))(i)?;

  Ok((
    i,
    Pair {
      sensor: Point {
        x: sensor_x,
        y: sensor_y,
      },
      beacon: Point {
        x: beacon_x,
        y: beacon_y,
      },
      manhattan_distance: (sensor_x - beacon_x).abs() + (sensor_y - beacon_y).abs(),
    },
  ))
}
