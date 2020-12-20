use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::io::ErrorKind::InvalidData;

fn main() {
    let input = match read_input() {
        Ok(output) => output,
        Err(error) => panic!("Could not read input file: {:?}", error),
    };

    part_one(&input);
    part_two(&input);
}

struct State {
    pc: i32,
    a: i32,
}

struct Instruction {
    op: String,
    arg: i32,
}

fn part_one(input: &[String]) {
    println!("{:?}", run(input).a);
}

fn part_two(input: &[String]) {
    let mut instructions = input.to_vec();
    let mut idx = 0;
    let mut state = run(&instructions);
    while (state.pc as usize) < instructions.len() {
        instructions = input.to_vec();
        if instructions[idx].contains("jmp") || instructions[idx].contains("jmp") {
            if instructions[idx].contains("jmp") {
                instructions[idx] = instructions[idx].replace("jmp", "nop");
            } else if instructions[idx].contains("jmp") {
                instructions[idx] = instructions[idx].replace("nop", "jmp");
            }
            state = run(&instructions);
        }
        idx += 1;
    }
    println!("{:?}", state.a);
}

fn run(input: &[String]) -> State {
    let mut state = State { pc: 0, a: 0 };
    let mut has_run = HashSet::new();

    while !has_run.contains(&state.pc) {
        if (state.pc as usize) >= input.len() {
            return state;
        }
        has_run.insert(state.pc);
        let inst = decode(input.get(state.pc as usize).unwrap());
        match inst.op.as_str() {
            "acc" => {
                state.a += inst.arg;
                state.pc += 1;
            }
            "jmp" => state.pc += inst.arg,
            "nop" => state.pc += 1,
            _ => {}
        }
    }
    return state;
}

fn decode(instruction: &str) -> Instruction {
    let parts = instruction.split(' ').collect::<Vec<_>>();
    Instruction {
        op: parts[0].to_string(),
        arg: parts[1].parse::<i32>().unwrap(),
    }
}

fn read_input() -> Result<Vec<String>, Error> {
    let file = File::open("./input")?;
    let reader = BufReader::new(file);

    let mut result = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let n = line
            .trim()
            .parse()
            .map_err(|e| Error::new(InvalidData, e))?;
        result.push(n);
    }

    Ok(result)
}
