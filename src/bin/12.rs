extern crate petgraph;
extern crate regex;

use std::io::{self, Read};
use std::collections::HashSet;
use regex::Regex;
use petgraph::Graph;
use petgraph::prelude::NodeIndex;

fn main() {
    let test_edges: Vec<(u32, u32)> = vec![
        (0, 2),
        (1, 1),
        (2, 0), (2, 3), (2, 4),
        (3, 2), (3, 4),
        (4, 2), (4, 3), (4, 6),
        (5, 6),
        (6, 4), (6, 5),
    ];

    let test_graph = Graph::<u32, u32>::from_edges(&test_edges);
    let c = connected_component(&NodeIndex::new(0), &test_graph);
    assert!(c.len() == 6);


    let test_data = "
0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5";
    assert!(parse_lines(test_data) == test_edges);


    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    let edges = parse_lines(&buffer);
    let graph = Graph::<u32, u32>::from_edges(&edges);
    let component = connected_component(&NodeIndex::new(0), &graph);
    println!("{:?}", component.len());

    let mut seen = HashSet::<NodeIndex>::new();
    let all_nodes: HashSet<NodeIndex> = graph.node_indices().collect();
    let mut components = Vec::<HashSet<NodeIndex>>::new();
    while seen.len() != graph.node_count() {
        // Clone to make the borrow checker happy when mutating seen below
        let unseen = all_nodes.difference(&seen).cloned().next().unwrap();
        let newly_seen = connected_component(&unseen, &graph);

        seen = seen.union(&newly_seen).cloned().collect();
        components.push(newly_seen);
    }

    println!("{:?}", components.len());
}

fn parse_lines(data: &str) -> Vec<(u32, u32)> {
    return data
        .split("\n")
        .filter(|&l| l != "")
        .flat_map(|l| parse_line(l))
        .collect();
}

fn parse_line(line: &str) -> Vec<(u32, u32)> {
    let re = Regex::new(r"\D+").unwrap();
    let entries: Vec<&str> = re.split(line).collect();
    let from = entries[0].parse::<u32>().unwrap();
    return entries
        .iter()
        .skip(1)
        .map(|to| (from, to.parse::<u32>().unwrap()))
        .collect();
}

fn connected_component(initial: &NodeIndex, g: &Graph<u32, u32>) -> HashSet<NodeIndex> {
    let mut stack = Vec::<NodeIndex>::new();
    let mut seen = HashSet::<NodeIndex>::new();

    stack.push(initial.clone());

    while let Some(x) = stack.pop() {
        if !seen.contains(&x) {
            seen.insert(x);
            for n in g.neighbors(x) {
                stack.push(n);
            }
        }
    }

    return seen;
}
