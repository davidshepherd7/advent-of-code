extern crate regex;
use std::collections::HashSet;
use regex::Regex;

use std::fs::File;
use std::io::prelude::*;

fn main() {

    let test_example = "
pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)
";


    let roots = find_root(&parse(test_example));
    assert!(roots == vec!["tknk"]);

    let mut f = File::open("7.in").expect("file not found");
    let mut puzzle_input = String::new();
    f.read_to_string(&mut puzzle_input).expect("blug");

    println!("find_root(&parse(puzzle_input)): {:?}", find_root(&parse(&puzzle_input.trim())));
}


fn find_root(data: &Vec<(String, u32, Vec<String>)>) -> Vec<String> {
    let nodes: HashSet<String> = data.iter().map(|&(ref n, _, _)| n.to_string()).collect();
    let children: HashSet<String> = data.iter().flat_map(|&(_, _, ref c)| c.clone()).collect();
    return nodes.difference(&children).cloned().collect();
}


fn parse(data: &str) -> Vec<(String, u32, Vec<String>)> {
    return data
        .trim()
        .split("\n")
        .map(parse_line)
        .collect();
}


fn parse_line(line: &str) -> (String, u32, Vec<String>) {
    let re = Regex::new(r"([a-z]+) \(([0-9]+)\)").unwrap();

    let mut temp = line.split("->");
    let cap = re.captures_iter(temp.next().unwrap()).next().unwrap();
    let this = &cap[1];
    let weight = cap[2].parse::<u32>().unwrap();

    let others: Vec<String> = match temp.next() {
        Some(value) => value.trim().split(",").map(|s| s.trim().to_string()).collect(),
        None => Vec::new(),
    };

    return (this.to_string(), weight, others);
}
