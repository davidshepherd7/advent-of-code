extern crate regex;

use std::collections::HashSet;
use std::collections::HashMap;
use regex::Regex;

use std::fs::File;
use std::io::prelude::*;

#[derive(Debug)]
struct Node {
    name: String,
    weight: u32,
    children: Vec<Node>,
    total_weight: u32,
}

#[derive(Debug)]
struct OffBalance {
    name: String,
    weight: u32,
    total_weight: u32,
    expected_total_weight: u32,
}

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
    let real_example = puzzle_input.trim();

    let data = parse(&real_example);
    let root = find_root(&data);
    println!("find_root(&parse(puzzle_input)): {:?}", root);

    let mut dict: HashMap<String, &(String, u32, Vec<String>)> = HashMap::new();
    for n in &data {
        dict.insert(n.0.clone(), n);
    }
    let tree = to_tree(dict.get(&root[0]).expect("no root in dict"), &dict);

    let x = tree_unbalanced(&tree);
    println!("x: {:?}", x);
    if let Some(off_balance) = x {
        let err: i32 = off_balance.expected_total_weight as i32 - off_balance.total_weight as i32;
        println!("solution: {:?}", off_balance.weight as i32 + err);
    }
}


fn find_root(data: &Vec<(String, u32, Vec<String>)>) -> Vec<String> {
    let nodes: HashSet<String> = data.iter().map(|&(ref n, _, _)| n.to_string()).collect();
    let children: HashSet<String> = data.iter().flat_map(|&(_, _, ref c)| c.clone()).collect();
    return nodes.difference(&children).cloned().collect();
}

fn find<'a>(name: &String, root: &'a Node) -> Option<&'a Node> {
    if *name == root.name {
        return Some(root);
    }
    return root.children.iter().flat_map(|c| find(&name, c)).next();
}


fn to_tree(root: &(String, u32, Vec<String>), dict: &HashMap<String, &(String, u32, Vec<String>)>) -> Node {
    let mut out = Node {
        name: root.0.clone(),
        weight: root.1,
        children: root.2.iter().map(|c| to_tree(dict.get(c).unwrap(), dict)).collect(),
        total_weight: 0,
    };

    out.total_weight = weight(&out);
    return out;
}


fn weight(root: &Node) -> u32 {
    return root.children.iter().map(|c| weight(&c)).sum::<u32>() + root.weight;
}


fn unbalanced(node: &Node) -> Option<OffBalance> {
    if node.children.len() == 0 {
        return None;
    }

    let mut counts = HashMap::<u32, u32>::new();
    for c in &node.children {
        let entry = counts.entry(c.total_weight).or_insert(0);
        *entry += 1;
    }
    let majority_weight = *counts.iter().max_by_key(|&(&_, &count)| count).unwrap().0;

    for child in &node.children {
        if child.total_weight != majority_weight {
            let out = OffBalance{
                name: child.name.clone(),
                weight: child.weight,
                total_weight: child.total_weight,
                expected_total_weight: majority_weight,
            };
            println!("found off balance node: {:?}", out);
            return Some(out);
        }
    }
    return None;
}


fn tree_unbalanced(root: &Node) -> Option<OffBalance> {
    root.children.iter()
        .map(|c| tree_unbalanced(c))
        .flat_map(|c| c.into_iter())
        .next()
        .or(unbalanced(root))
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
