

fn main() {
    println!("{:?}", parse("{<a>,<a>,<a>,<a>}"));
}


fn parse(data: &str) {
    let final_state = data.chars().scan((false, false), |state, ch| {
        println!("{:?}", state);
        if state.0 {
            *state = (false, false);
        }
        else if state.1 {
            if ch == '>' {
                *state = (false, false);
            }
            else {
                *state = (false, true);
            }
        }
        else {
            if ch == '<' {
                *state = (false, true);
            }
            else if ch == '{' {
                // TODO: push onto stack, or something
            }
            else {
                println!("valid character: {:?}", ch);
                *state = (false, false);
            }
        }

        return Some(*state);
    }).last();

    println!("{:?}", final_state);
}
