fn main() {
    assert!(f("1122") == 3);
    assert!(f("1111") == 4);
    assert!(f("1234") == 0);
    assert!(f("91212129") == 9);

    assert!(g("1212") == 6);
    assert!(g("1221") == 0);
    assert!(g("123425") == 4);
    assert!(g("123123") == 12);
    assert!(g("12131415") == 4);
}

fn base(string: &str, forward: usize) -> u32 {
    let n = string.len();
    let cycled = string.chars()
        .cycle()
        .skip(forward)
        .take(n)
        ;

    return string.chars()
        .zip(cycled)
        .filter( | p | p.0 == p.1)
        .map( | p | char::to_digit(p.0, 10).unwrap())
        .sum::<u32>()
        ;
}


fn f(string: &str) -> u32 {
    base(string, 1)
}


fn g(string: &str) -> u32 {
    base(string, string.len() / 2)
}
