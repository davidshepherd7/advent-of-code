use std::collections::HashMap;

use std::io::{self, Read};

#[derive(Debug)]
struct Instruction<'a> {
    operand: &'a str,
    operation: &'a str,
    value: i64,
    conditional_operand: &'a str,
    conditional_operation: &'a str,
    conditional_value: i64,
}

type Registers<'a> = HashMap<&'a str, i64>;


fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let instructions = parse(&buffer);
    let final_registers = run_program(&instructions);
    println!("registers: {:?}", final_registers.0);
    println!("max value ever: {:?}", final_registers.1);
    println!("max value at end: {:?}", final_registers.0.values().max());
}


fn parse(lines: &str) -> Vec<Instruction> {
    return lines.trim().split("\n").map(|l| l.trim()).map(|l| parse_line(l)).collect();
}


fn parse_line(line: &str)-> Instruction {
    let data: Vec<&str> = line.split(" ").collect();

    if data.len() != 7 {
        panic!("Unexpected number of fields in line {}", line);
    }

    let value = match data[2].parse() {
        Result::Ok(val) => val,
        Result::Err(_) => panic!("Failed to parse {} as op value", data[2]),
    };

    let condition_value = match data[6].parse() {
        Result::Ok(val) => val,
        Result::Err(_) => panic!("Failed to parse {} as conditional value", data[6]),
    };

    return Instruction {
        operand: data[0],
        operation: data[1],
        value: value,
        conditional_operand: data[4],
        conditional_operation: data[5],
        conditional_value: condition_value,
    };
}


fn run_program<'a>(insts: &Vec<Instruction<'a>>) -> (Registers<'a>, i64) {

    let mut registers = Registers::new();
    let mut max: i64 = 0;

    for inst in insts.iter() {
        run_instruction(&inst, &mut registers);
        match registers.values().max() {
            Some(m) => max = std::cmp::max(max, *m),
            None => (),
        }
    }

    return (registers, max);
}


fn run_instruction<'a>(inst: &Instruction<'a>, registers: &mut Registers<'a>) {
    if should_run(&inst, &registers) {
        let reg = registers.entry(inst.operand).or_insert(0);
        if inst.operation == "inc" {
            *reg += inst.value;
        }
        else if inst.operation == "dec" {
            *reg -= inst.value;
        }
        else {
            panic!("unknown operation {}", inst.operation);
        }
    }
}


fn should_run<'a>(inst: &Instruction<'a>, registers: &Registers<'a>) -> bool {
    let operand = registers.get(inst.conditional_operand).unwrap_or(&0);

    match inst.conditional_operation {
        "<" => operand < &inst.conditional_value,
        ">" => operand > &inst.conditional_value,
        "==" => operand == &inst.conditional_value,
        "!=" => operand != &inst.conditional_value,
        "<=" => operand <= &inst.conditional_value,
        ">=" => operand >= &inst.conditional_value,
        _ => panic!("unknown conditional operator {}", inst.conditional_operation),
    }
}
