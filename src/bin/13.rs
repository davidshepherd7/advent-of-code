
extern crate itertools;

use std::io::{self, Read};

use std::collections::HashMap;


#[derive(Debug)]
#[derive(Clone)]
struct State {
    pos: usize,
    caught: bool,
    delay: usize,
}

type Firewall = HashMap<usize, usize>;

impl State {
    fn new(firewall: &Firewall, delay: usize) -> State {
        let position = 0;
        return State {
            pos: position,
            caught: check_if_caught(&firewall, position, delay),
            delay: delay,
        };
    }
}

fn check_if_caught(firewall: &Firewall, current_position: usize, delay: usize) -> bool {
    return match firewall.get(&current_position) {
        Some(range) => (current_position + delay) % (2 * (range - 1)) == 0,
        None => false,
    };
}


fn step(prev: &State, firewall: &Firewall) -> State {
    return State {
        pos: prev.pos + 1,
        caught: check_if_caught(&firewall, prev.pos + 1, prev.delay),
        delay: prev.delay,
    };
}

fn caught(s: &&State) -> bool {
    return s.caught;
}

fn cost(s: &State, firewall: &Firewall) -> usize {
    match firewall.get(&s.pos) {
        Some(range) => range * s.pos,
        None => 0,
    }
}

fn run_simulation(firewall: &Firewall, delay: usize) -> Vec<State> {
    let s = State::new(&firewall, delay);

    let mut states = Vec::<State>::new();
    states.push(s);
    for _ in 0..*firewall.keys().max().unwrap() {
        let next = step(&states.last().unwrap(), &firewall);
        states.push(next);
    }

    return states;
}

fn total_cost(firewall: &Firewall, delay: usize) -> usize {
    return run_simulation(&firewall, delay).iter()
        .filter(caught)
        .map(|x| cost(&x, &firewall))
        .sum::<usize>();
}

fn ever_caught(firewall: &Firewall, delay: usize) -> bool {
    return run_simulation(&firewall, delay).iter().any(|state| state.caught);
}


fn min_delay(firewall: &Firewall) -> usize {
    let delays = 0..;
    return delays
        .map(|d|{ println!("{:?}", d); return d;})
        .filter(|&d| !ever_caught(&firewall, d))
        .next()
        .unwrap();
}


fn parse(data: &str) -> Firewall {
    let mut out = Firewall::new();

    for line in data.split("\n").filter(|&l| l != "") {
        let l: Vec<&str> = line.split(": ").collect();
        out.insert(l[0].parse::<usize>().unwrap(), l[1].parse::<usize>().unwrap());
    }

    return out;
}


fn main() {
    let firewall: HashMap<usize, usize> = [(0, 3), (1, 2), (4, 4), (6, 4)].iter().cloned().collect();
    println!("{:?}", firewall);
    assert!(total_cost(&firewall, 0) == 24);
    assert!(min_delay(&firewall) == 10);

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let real = parse(&buffer);
    let cost = total_cost(&real, 0);
    assert!(cost == 2264);

    println!("{:?}", min_delay(&real));
}
