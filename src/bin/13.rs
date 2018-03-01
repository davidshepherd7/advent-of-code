
extern crate itertools;

use std::io::{self, Read};

use std::collections::HashMap;

type Firewall = HashMap<usize, usize>;

#[derive(Debug)]
#[derive(Clone)]
struct State {
    pos: usize,
    caught: bool,
    delay: usize,
    firewall: Firewall,
    size: usize,
}

impl Iterator for State {
    type Item = (usize, bool);
    fn next(&mut self) -> Option<(usize, bool)> {
        if self.pos >= self.size {
            return None;
        }

        let out = Some((self.pos, self.caught));

        self.pos = self.pos + 1;
        self.caught = check_if_caught(&self.firewall, self.pos, self.delay);

        return out;
    }
}

impl State {
    fn new(firewall: &Firewall, delay: usize) -> State {
        return State {
            pos: 0,
            caught: check_if_caught(&firewall, 0, delay),
            delay: delay,
            firewall: firewall.clone(),
            size: *firewall.keys().max().unwrap() + 1,
        };
    }
}

fn check_if_caught(firewall: &Firewall, current_position: usize, delay: usize) -> bool {
    return match firewall.get(&current_position) {
        Some(range) => (current_position + delay) % (2 * (range - 1)) == 0,
        None => false,
    };
}

fn run_simulation(firewall: &Firewall, delay: usize) -> State {
    return State::new(&firewall, delay);
}

fn total_cost(firewall: &Firewall, delay: usize) -> usize {
    return run_simulation(&firewall, delay)
        .filter(|&(_, caught)| caught)
        .map(|(pos, _)|
             match firewall.get(&pos) {
                 Some(range) => range * pos,
                 None => 0,
             }
        )
        .sum::<usize>();
}

fn ever_caught(firewall: &Firewall, delay: usize) -> bool {
    return run_simulation(&firewall, delay).any(|(_, caught)| caught);
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
    println!("min delay {:?}", min_delay(&firewall));
    assert!(min_delay(&firewall) == 10);

    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let real = parse(&buffer);
    let cost = total_cost(&real, 0);
    assert!(cost == 2264);

    println!("{:?}", min_delay(&real));
}
