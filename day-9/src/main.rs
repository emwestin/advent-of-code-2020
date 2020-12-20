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

fn part_one(input: &[String]) {
    let numbers = input
        .iter()
        .map(|i| i.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    for i in 0..(numbers.len() - 25) {
        let mut preamble: Vec<i64> = numbers.iter().skip(i).take(25).map(|n| *n as i64).collect();
        let mut valid = false;
        while !preamble.is_empty() {
            let current = preamble.pop().unwrap();
            valid |= preamble.contains(&(numbers[i + 25] - current));
        }
        if !valid {
            println!("{:?}", numbers[i + 25]);
        }
    }
}

fn part_two(input: &[String]) {
    let numbers = input
        .iter()
        .map(|i| i.parse::<i64>().unwrap())
        .collect::<Vec<i64>>();

    for i in 0..numbers.len() {
        let mut length = 2;
        while numbers.iter().skip(i).take(length).sum::<i64>() < 1492208709 {
            length += 1;
        }

        let set = numbers
            .iter()
            .skip(i)
            .take(length)
            .map(|n| *n as i64)
            .collect::<Vec<i64>>();
        if set.iter().sum::<i64>() == 1492208709 {
            let min = set.iter().min().unwrap();
            let max = set.iter().max().unwrap();
            println!("{:?}", min + max);
        }
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
