extern crate regex;

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



    let out=parse(&test_example);
    println!("out: {:?}", out);
}

fn parse(data: &str) -> Vec<(String, u32, Vec<String>)> {
    return data
        .trim()
        .split("\n")
        .map(parse_line)
        .collect();
}

fn parse_line(line: &str) -> (String, u32, Vec<String>) {
    let mut temp = line.split("->");
    let this_data = temp.next().unwrap();
    let this = this_data.split("(").next().unwrap().trim();

    let others: Vec<String> = match temp.next() {
        Some(value) => value.trim().split_whitespace().map(|s| s.to_string()).collect(),
        None => Vec::new(),
    };

    return (this.to_string(), 0, others);
}
