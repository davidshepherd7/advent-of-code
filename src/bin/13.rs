
extern crate itertools;

use std::io::{self, Read};


#[derive(Debug)]
#[derive(Clone)]
struct State {
    pos: i64,
    scanning: Vec<Option<u32>>,
    caught: bool,
}

type Firewall = Vec<Option<u32>>;

impl State {
    fn new(firewall: &Firewall, delay: u32) -> State {
        let position = -1 * (delay as i64);
        let scanning = firewall.iter().map(|f| match f {
                &Some(_) => Some(0),
                &None => None
            }).collect();
        return State {
            pos: position,
            caught: check_if_caught(&scanning, position),
            scanning: scanning,
        };
    }
}

fn progress_layer(currently_scanning: &Option<u32>, range: &Option<u32>) -> Option<u32> {
    if currently_scanning.is_none() || range.is_none() {
        return None;
    }
    else {
        let location = currently_scanning.clone().unwrap();
        return Some((location + 1) % (2* (range.unwrap() - 1)));
    }
}

fn check_if_caught(scanning: &Vec<Option<u32>>, current_position: i64) -> bool {
    if current_position < 0 {
        return false;
    }
    else {
        let t: usize = current_position as usize;
        return !scanning[t].is_none()
            && scanning[t].clone().unwrap() == 0;
    }
}


fn step(prev: &State, firewall: &Firewall) -> State {
    let scanning: Vec<Option<u32>> = prev.scanning.iter()
        .cloned()
        .zip(firewall)
        .map(|(s, f)| progress_layer(&s, &f))
        .collect();
    return State {
        pos: prev.pos + 1,
        caught: check_if_caught(&scanning, prev.pos + 1),
        scanning: scanning,
    };
}

fn caught(s: &&State) -> bool {
    return s.caught;
}

fn cost(s: &State, firewall: &Firewall) -> u32 {
    assert!(s.pos >= 0, "Cannot be caught at negative positions but postion is '{:?}'", s.pos);
    if let Some(range) = firewall[s.pos as usize] {
        return range * (s.pos as u32);
    }
    else {
        return 0;
    }
}

fn run_simulation(firewall: &Firewall, delay: u32) -> Vec<State> {
    let s = State::new(&firewall, delay);

    let mut states = Vec::<State>::new();
    states.push(s);
    for _ in 1..(firewall.len() + delay as usize) {
        let prev = &states.last().cloned().unwrap();
        states.push(step(&prev, &firewall));

        if states.last().unwrap().caught {
            println!("caught at {:?}", states.last().unwrap().pos);
        }
    }

    return states;
}

fn total_cost(firewall: &Firewall, delay: u32) -> u32 {
    return run_simulation(&firewall, delay).iter()
        .filter(caught)
        .map(|x| cost(&x, &firewall))
        .sum::<u32>();
}

fn ever_caught(firewall: &Firewall, delay: u32) -> bool {
    return run_simulation(&firewall, delay).iter().any(|state| state.caught);
}


fn min_delay(firewall: &Firewall) -> u32 {
    let delays = 0..;
    return delays
        .map(|d|{ println!("{:?}", d); return d;})
        .filter(|&d| !ever_caught(&firewall, d))
        .next()
        .unwrap();
}


fn parse(data: &str) -> Firewall {
    let mut out = Firewall::new();
    out.resize(85, None);

    for line in data.split("\n").filter(|&l| l != "") {
        let l: Vec<&str> = line.split(": ").collect();
        out[l[0].parse::<usize>().unwrap()] = Some(l[1].parse::<u32>().unwrap());
    }

    return out;
}


fn main() {
    let firewall = vec![Some(3), Some(2), None, None, Some(4), None, Some(4)];
    assert!(total_cost(&firewall, 0) == 24);
    assert!(min_delay(&firewall) == 10);

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let real = parse(&buffer);
    let cost = total_cost(&real, 0);
    assert!(cost == 2264);

    println!("{:?}", min_delay(&real));
}
