use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::io::ErrorKind::InvalidData;

const SUM: i32 = 2020;

fn main() {
    let input = match read_input() {
        Ok(output) => output,
        Err(error) => panic!("Could not read input file: {:?}", error),
    };

    part_one(&input);
    part_two(&input);
}

fn part_one(input: &[i32]) {
    match find_sum(input, SUM) {
        Some(s) => println!("{}", s.0 * s.1),
        None => println!("Can't find values that sum to {}", SUM),
    }
}

fn part_two(input: &[i32]) {
    for (i, el) in input.iter().enumerate() {
        let result = find_sum(&input[i..input.len()], SUM - el);
        match result {
            Some(s) => println!("{}", el * s.0 * s.1),
            None => continue,
        }
    }
}

fn find_sum(values: &[i32], sum: i32) -> Option<(i32, i32)> {
    let mut values = values.to_vec();
    values.sort();

    while !values.is_empty() {
        let current = values.pop().unwrap();
        if values.contains(&(sum - current)) {
            return Some((current, (sum - current)));
        }
    }

    None
}

fn read_input() -> Result<Vec<i32>, Error> {
    let file = File::open("./input")?;
    let reader = BufReader::new(file);

    let mut result = Vec::new();
    for num in reader.lines() {
        let num = num?;
        let n = num.trim().parse().map_err(|e| Error::new(InvalidData, e))?;
        result.push(n);
    }

    Ok(result)
}
