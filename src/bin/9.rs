use std::io::{self, Read};


type CharIter<'a> = Box<Iterator<Item = char> + 'a>;


fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();
    println!("{:?}", total_score(Box::new(buffer.chars())));

    assert!(total_score(Box::new("{}".chars())).0 == 1);
    assert!(total_score(Box::new("{{{}}}".chars())).0 == 6);
    assert!(total_score(Box::new("{{},{}}".chars())).0 == 5);
    assert!(total_score(Box::new("{{{},{},{{}}}}".chars())).0 == 16);
    assert!(total_score(Box::new("{<a>,<a>,<a>,<a>}".chars())).0 == 1);
    assert!(total_score(Box::new("{{<ab>},{<ab>},{<ab>},{<ab>}}".chars())).0 == 9);
    assert!(total_score(Box::new("{{<!!>},{<!!>},{<!!>},{<!!>}}".chars())).0 == 9);
    assert!(total_score(Box::new("{{<a!>},{<a!>},{<a!>},{<ab>}}".chars())).0 == 3);
}

fn strip_cancelled<'a>(data: CharIter<'a>) -> CharIter<'a> {
    let mut cancelled = false;
    let out = data
        .filter(move |&c| {
            if cancelled {
                cancelled = false;
                return false;
            }
            else if c == '!' {
                cancelled = true;
                return false;
            }
            else {
                return true;
            }
        });

    return Box::new(out);
}


fn strip_garbage<'a>(data: CharIter<'a>) -> (CharIter<'a>, u64) {
    let mut garbage_count = 0;
    let mut garbage = false;
    let out = data.filter(move |&d| {
        if garbage {
            if d == '>' {
                garbage = false;
            }
            else {
                garbage_count += 1;
            }
            return false;
        }
        else if d == '<' {
            garbage = true;
            return false;
        }
        else {
            return true;
        }
    });

    return ( Box::new(out), garbage_count );
}


fn scores(data: CharIter) -> Vec<u64> {
    let mut out = Vec::<u64>::new();

    let mut depth = 0;
    for d in data {
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


fn total_score(data: CharIter) -> (u64, u64) {
    let (clean, garbage_count) = strip_garbage(strip_cancelled(data));
    return (scores(clean).iter().sum(), garbage_count);
}
