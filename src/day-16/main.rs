use std::collections::HashMap;

use nom::branch::alt;
use nom::bytes::complete::take_while;
use nom::character::complete as cc;
use nom::combinator::{all_consuming, map_res};
use nom::multi::separated_list1;
use nom::sequence::{separated_pair, tuple};
use nom::{bytes::complete::tag, Finish, IResult};
use petgraph::algo::floyd_warshall;
use petgraph::{prelude::*, Directed, Graph};

#[derive(Debug, Clone)]
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

  let mut graph: Graph<(), (), Directed> = Graph::new();
  let node_names = HashMap::from_iter(valves.iter().map(|valve| (&valve.name, graph.add_node(()))));
  let node_indexes: HashMap<usize, String> = HashMap::from_iter(
    valves
      .iter()
      .enumerate()
      .map(|valve| (valve.0, valve.1.name.clone())),
  );

  let graph = get_graph(valves.clone(), graph, node_names.clone());
  let weight_map = get_weight_map(valves.clone(), node_names.clone());

  let distances = floyd_warshall(&graph, |edge| {
    if let Some(weight) = weight_map.get(&(edge.source(), edge.target())) {
      *weight
    } else {
      unreachable!()
    }
  })
  .unwrap();

  println!("Total paths found: {:?}", distances.len());
  distances.iter().for_each(|node| {
    let node_index_pair = node.0;
    let weight = node.1;
    println!(
      "Paths found {:?} {:?} with weight {}",
      node_indexes.get(&node_index_pair.0.index()).unwrap(),
      node_indexes.get(&node_index_pair.1.index()).unwrap(),
      weight
    );
  });
}

fn get_weight_map(
  valves: Vec<Valve>,
  nodes: HashMap<&String, NodeIndex>,
) -> HashMap<(NodeIndex, NodeIndex), i32> {
  let mut valves_graph_weight: HashMap<(NodeIndex, NodeIndex), i32> = HashMap::new();
  valves.iter().for_each(|valve| {
    valves_graph_weight.insert(
      (
        *nodes.get(&valve.name).unwrap(),
        *nodes.get(&valve.name).unwrap(),
      ),
      0,
    );
    valve.tunnel_valves.iter().for_each(|tunnel_valve_name| {
      valves_graph_weight.insert(
        (
          *nodes.get(&valve.name).unwrap(),
          *nodes.get(&tunnel_valve_name).unwrap(),
        ),
        valve.flow_rate,
      );
    })
  });
  return valves_graph_weight;
}

fn get_graph(
  valves: Vec<Valve>,
  mut graph: Graph<(), ()>,
  node_names: HashMap<&String, NodeIndex>,
) -> Graph<(), ()> {
  let mut valves_graph: Vec<(NodeIndex, NodeIndex)> = Vec::new();
  valves.iter().for_each(|valve| {
    valve.tunnel_valves.iter().for_each(|tunnel_valve_name| {
      valves_graph.push((
        *node_names.get(&valve.name).unwrap(),
        *node_names.get(&tunnel_valve_name).unwrap(),
      ));
    })
  });
  graph.extend_with_edges(&valves_graph);
  return graph;
}

fn parse_all_valves(i: &str) -> IResult<&str, Vec<Valve>> {
  return separated_list1(cc::newline, parse_valve)(i);
}

fn parse_valve(input: &str) -> IResult<&str, Valve> {
  // Sample input:
  // Valve QJ has flow rate=11; tunnels lead to valves HB, GL
  let (input, mut result) = separated_pair(
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

  #[rustfmt::skip]
  result.1.3.sort();

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
