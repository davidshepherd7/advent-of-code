use std::ops::BitXor;

fn main() {
    assert!(hash1(vec![3, 4, 1, 5].into_iter(), &vec![0, 1, 2, 3, 4], 0, 0) == (4, vec![3, 4, 2, 1, 0]));

    let real_lengths = vec![227, 169, 3, 166, 246, 201, 0, 47, 1, 255, 2, 254, 96, 3, 97, 144];
    let out = hash(real_lengths);
    assert!(out[0] * out[1] == 13760);

    let emp = dense_hash(&hash(lengths("")));
    assert!(emp == "a2582a3a0e66e6e86e3812dcb672a272");

    let aoc = dense_hash(&hash(lengths("AoC 2017")));
    assert!(aoc == "33efeb34ea91902bb2f59c9920caa6cd");

    let result = dense_hash(&hash(lengths("227,169,3,166,246,201,0,47,1,255,2,254,96,3,97,144")));
    println!("hash: {:?}", result);
}

fn lengths(text: &str) -> Vec<usize> {
    let text = text_to_ints(text);
    let extra = vec![17, 31, 73, 47, 23];

    let nlengths = text.len() + extra.len();

    return text.into_iter().chain(extra.into_iter())
        .cycle()
        .take(nlengths * 64)
        .collect();
}

fn hash<I>(lengths: I) -> Vec<usize>
    where I: IntoIterator<Item=usize>
{
    return hash1(lengths.into_iter(), &(0..256).collect(), 0, 0).1;
}

fn hash1<I>(mut lengths: I, initial: &Vec<usize>, start: usize, skip: usize) -> (usize, Vec<usize>)
        where I: Iterator<Item=usize>
{
    let length = match lengths.next() {
        Some(x) => x,
        None => return (start, initial.clone()),
    };

    let mut new = initial.clone();

    for i in 0..length {
        let source_idx = (start + length - (i + 1)) % initial.len();
        let target_idx = (start + i) % initial.len();
        new[target_idx] = initial[source_idx];
    }

    return hash1(lengths, &new, (start + length + skip) % initial.len(), skip + 1);
}


fn text_to_ints(text: &str) -> Vec<usize> {
    return text.chars().map(|c| c as usize).collect();
}


fn dense_hash(sparse_hash: &Vec<usize>) -> String {
    return sparse_hash
        .chunks(16)
        .map(|w| w.into_iter().fold(0 as u8, |acc, x| acc.bitxor(*x as u8)))
        .map(|x| format!("{:02x}", x))
        .collect();
}
