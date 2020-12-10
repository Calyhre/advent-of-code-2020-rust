use std::time::Instant;

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::prelude::*;

use petgraph::{
    graph::{Edges, NodeIndex},
    visit::EdgeRef,
    Directed,
    EdgeDirection::{self, Incoming, Outgoing},
    Graph,
};

fn read_logs() -> String {
    let mut content = String::new();
    match File::open("inputs/day_07.txt") {
        Ok(mut file) => {
            file.read_to_string(&mut content).unwrap();
        }
        Err(_error) => panic!("Error opening file"),
    }
    content
}

fn build_graph<'a>(
    graph: &'a mut Graph<&'a str, usize>,
    logs: &'a String,
) -> (&'a mut Graph<&'a str, usize>, NodeIndex) {
    let mut nodes = HashMap::new();

    let mut lines = logs.split("\n");
    while let Some(line) = lines.next() {
        let rules: Vec<&'a str> = line.splitn(2, " bags contain ").collect();
        let bag_color: &'a str = rules[0];
        let parent = *nodes
            .entry(bag_color.to_owned())
            .or_insert_with(|| graph.add_node(bag_color));

        let mut children = rules[1].split(", ");

        while let Some(child) = children.next() {
            if child == "no other bags" || child == "no other bags." {
                continue;
            }
            let color: &str;
            if child.ends_with(" bags.") {
                color = child.clone().strip_suffix(" bags.").unwrap();
            } else if child.ends_with(" bags") {
                color = child.clone().strip_suffix(" bags").unwrap();
            } else if child.ends_with(" bag.") {
                color = child.clone().strip_suffix(" bag.").unwrap();
            } else {
                color = child.clone().strip_suffix(" bag").unwrap();
            }
            let mut color = color.splitn(2, ' ');
            let weight = color.next().unwrap();
            let weight = weight.parse::<usize>().unwrap();
            let child_color = color.next().unwrap();

            let child = *nodes
                .entry(child_color.to_owned())
                .or_insert_with(|| graph.add_node(child_color));
            graph.add_edge(parent, child, weight);
        }
    }
    (graph, *nodes.get("shiny gold").unwrap())
}

fn find_leaves(
    result: &mut HashSet<NodeIndex>,
    graph: &Graph<&str, usize>,
    edges: Edges<usize, Directed>,
    dir: EdgeDirection,
) -> usize {
    edges
        .map(|edge| {
            let node = match dir {
                Incoming => edge.source(),
                Outgoing => edge.target(),
            };
            let edges = graph.edges_directed(node, dir);
            result.insert(node);
            find_leaves(result, &graph, edges, dir) * edge.weight()
        })
        .sum::<usize>()
        + 1
}

pub fn part_one() -> usize {
    let logs = read_logs();

    let mut bags = Graph::<&str, usize, Directed, u32>::new();
    let (bags, shiny_gold) = build_graph(&mut bags, &logs);

    let mut result: HashSet<NodeIndex> = HashSet::new();
    let edges = bags.edges_directed(shiny_gold, Incoming);
    find_leaves(&mut result, &bags, edges, Incoming);

    result.len()
}

pub fn part_two() -> usize {
    let logs = read_logs();

    let mut bags = Graph::<&str, usize, Directed, u32>::new();
    let (bags, shiny_gold) = build_graph(&mut bags, &logs);

    let mut result: HashSet<NodeIndex> = HashSet::new();
    let edges = bags.edges_directed(shiny_gold, Outgoing);

    let leaves = find_leaves(&mut result, &bags, edges, Outgoing);
    leaves - 1
}

pub fn main() {
    let now = Instant::now();
    println!("Day 7: Handy Haversacks");
    println!("  Part one: {}", part_one());
    println!("  Part two: {}", part_two());
    println!("  Time: {}ms", now.elapsed().as_millis());
}
