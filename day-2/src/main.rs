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
    let count: usize = input
        .iter()
        .map(String::to_string)
        .map(self::parse_line)
        .filter(|x| (x.0..=x.1).contains(&(x.3.matches(x.2).count())))
        .count();

    println!("{}", count);
}

fn part_two(input: &[String]) {
    let count: usize = input
        .iter()
        .map(String::to_string)
        .map(self::parse_line)
        .filter(|x| {
            is_char_at(&x.3, x.2, x.0) && !is_char_at(&x.3, x.2, x.1)
                || !is_char_at(&x.3, x.2, x.0) && is_char_at(&x.3, x.2, x.1)
        })
        .count();

    println!("{}", count);
}

fn is_char_at(value: &String, character: char, index: usize) -> bool {
    value.chars().nth(index).unwrap() == character
}

fn parse_line(line: String) -> (usize, usize, char, String) {
    let whole: Vec<&str> = line.split(':').collect();
    let rule: Vec<&str> = whole[0].split(' ').collect();
    let range: Vec<&str> = rule[0].split('-').collect();
    let lower = range[0].parse::<usize>().unwrap();
    let higher = range[1].parse::<usize>().unwrap();
    let letter = rule[1].chars().nth(0).unwrap();

    (lower, higher, letter, whole[1].to_string())
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
