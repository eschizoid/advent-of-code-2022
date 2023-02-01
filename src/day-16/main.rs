use nom::branch::alt;
use nom::bytes::complete::take_while;
use nom::character::complete as cc;
use nom::combinator::{all_consuming, map_res};
use nom::multi::separated_list1;
use nom::sequence::{separated_pair, tuple};
use nom::{bytes::complete::tag, Finish, IResult};
use std::collections::HashMap;

#[derive(Debug)]
struct Valve {
  name: String,
  flow_rate: i32,
  tunnel_valves: Vec<String>,
}

fn main() {
  let valves = all_consuming(parse_all_valves)(include_str!("input.txt"))
    .finish()
    .unwrap()
    .1;
  println!("{:?}", valves);
}

fn build_valve_map(valves: Vec<Valve>) -> HashMap<String, Vec<String>> {
  let mut valve_map = HashMap::new();
  for valve in valves {
    valve_map.insert(valve.name, valve.tunnel_valves);
  }
  return valve_map;
}

fn parse_all_valves(i: &str) -> IResult<&str, Vec<Valve>> {
  return separated_list1(cc::newline, parse_valve)(i);
}

fn parse_valve(input: &str) -> IResult<&str, Valve> {
  // Valve QJ has flow rate=11; tunnels lead to valves HB, GL
  let (input, result) = separated_pair(
    tuple((
      tag("Valve "),
      map_res(take_while(|c: char| c.is_alphabetic()), |s: &str| {
        s.parse::<String>()
      }),
      tag(" has flow rate="),
      map_res(take_while(|c: char| c.is_numeric()), |s: &str| {
        s.parse::<i32>()
      }),
    )),
    tag("; "),
    tuple((
      alt((tag("tunnels "), tag("tunnel "))),
      alt((tag("lead "), tag("leads "))),
      alt((tag("to valves "), tag("to valve "))),
      separated_list1(
        tag(", "),
        map_res(take_while(|c: char| c.is_alphabetic()), |s: &str| {
          s.parse::<String>()
        }),
      ),
    )),
  )(input)?;

  Ok((
    input,
    Valve {
      #[rustfmt::skip]
      name: result.0.1,
      #[rustfmt::skip]
      flow_rate: result.0.3,
      #[rustfmt::skip]
      tunnel_valves: result.1.3,
    },
  ))
}
