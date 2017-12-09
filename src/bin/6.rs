use std::collections::HashMap;

fn main() {
    assert!(count_redistributions(vec![0, 2, 7, 0]) == (5, 4));

    let banks = vec![11, 11, 13, 7, 0, 15, 5, 5, 4, 4, 1, 1, 7, 1, 15, 11];
    println!("count_redistributions(banks): {:?}", count_redistributions(banks));
}

fn count_redistributions(mut banks: Vec<u32>) -> (usize, usize) {
    let mut seen: HashMap<Vec<u32>, usize> = HashMap::new();
    println!("banks: {:?}", banks);
    while !seen.contains_key(&banks) {
        let seen_count = seen.len();
        seen.insert(banks.clone(), seen_count);
        redist(&mut banks);
        println!("banks: {:?}", banks);
    }

    return (seen.len(), seen.len() - seen.get(&banks).unwrap());
}

fn redist(banks: &mut Vec<u32>) {
    let max: u32 = banks.iter().max().unwrap().clone();
    let from = banks.iter().enumerate().filter(|&(_, &x)| x == max).next().unwrap().0;
    let mut from_value = banks[from];
    banks[from] = 0;

    let n = banks.len();

    let mut add_to = (from + 1) % n;
    while from_value > 0 {
        banks[add_to] += 1;
        from_value -= 1;
        add_to = (add_to + 1) % n;
    }
}
