use std::io::{self, Read};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let clean = strip_garbage(buffer.as_str());
    println!("{:?}", clean);
    println!("{:?}", total_score(buffer.as_str()));


    assert!(total_score("{}").0 == 1);
    assert!(total_score("{{{}}}").0 == 6);
    assert!(total_score("{{},{}}").0 == 5);
    assert!(total_score("{{{},{},{{}}}}").0 == 16);
    assert!(total_score("{<a>,<a>,<a>,<a>}").0 == 1);
    assert!(total_score("{{<ab>},{<ab>},{<ab>},{<ab>}}").0 == 9);
    assert!(total_score("{{<!!>},{<!!>},{<!!>},{<!!>}}").0 == 9);
    assert!(total_score("{{<a!>},{<a!>},{<a!>},{<ab>}}").0 == 3);
}


fn strip_garbage(data: &str) -> (String, u64) {

    let mut v = Vec::<char>::new();
    let mut garbage_count = 0;

    let mut skip = false;
    let mut garbage = false;
    for d in data.chars() {
        if skip {
            skip = false;
            continue;
        }
        else if d == '!' {
            skip = true;
            continue;
        }
        else if garbage {
            if d == '>' {
                garbage = false;
                continue;
            }
            else {
                garbage_count += 1;
                continue;
            }
        }
        else if d == '<' {
            garbage = true;
            continue;
        }
        else {
            v.push(d);
            continue;
        }
    }

    return ( v.into_iter().collect(), garbage_count );
}


fn scores(data: &str) -> Vec<u64> {
    let mut out = Vec::<u64>::new();

    let mut depth = 0;
    for d in data.chars() {
        if d == '{' {
            depth += 1;
        }
        else if d == '}' {
            out.push(depth);
            depth -= 1;
        }
    }

    return out;
}

fn total_score(data: &str) -> (u64, u64) {
    let (clean, garbage_count) = strip_garbage(data);
    return (scores(clean.as_str()).iter().sum(), garbage_count);
}
