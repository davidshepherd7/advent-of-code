use std::fs::File;
use std::io::prelude::*;

fn main() {
    assert!(nsteps(vec![0, 3, 0, 1, -3], part1_step) == 5);
    assert!(nsteps(vec![0, 3, 0, 1, -3], part2_step) == 10);

    let mut f = File::open("5.in").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("blug");
    let jump_list: Vec<i32> = contents
        .trim()
        .split("\n")
        .map(|s| s.parse::<i32>().unwrap())
        .collect();

    println!("nsteps(jump_list, part1_step): {:?}", nsteps(jump_list.clone(), part1_step));
    println!("nsteps(jump_list, part2_step): {:?}", nsteps(jump_list.clone(), part2_step));
}


fn nsteps(mut jump_list: Vec<i32>, step_function: fn(usize, &mut Vec<i32>) -> i32) -> u32 {
    let mut counter: i32 = 0;
    let mut i: u32 = 0;
    while counter >= 0 && (counter as usize) < jump_list.len() {
        counter = step_function(counter as usize, &mut jump_list);
        i = i + 1;
    }
    return i;
}

fn part1_step(counter: usize, jump_list: &mut Vec<i32>) -> i32 {
    let jump = jump_list[counter];
    jump_list[counter] += 1;
    return (counter as i32) + jump;
}

fn part2_step(counter: usize, jump_list: &mut Vec<i32>) -> i32 {
    let jump = jump_list[counter];
    if jump >= 3 {
        jump_list[counter] -= 1;
    }
    else {
        jump_list[counter] += 1;
    }
    return (counter as i32) + jump;
}
