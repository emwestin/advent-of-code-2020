use regex::Regex;
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
    let passports = transform_passports(&input.to_vec());
    let result = passports
        .iter()
        .filter(|p| is_passport_valid(p, false))
        .count();
    println!("{}", result);
}

fn part_two(input: &[String]) {
    let passports = transform_passports(&input.to_vec());
    let result = passports
        .iter()
        .filter(|p| is_passport_valid(p, true))
        .count();
    println!("{}", result);
}

fn is_passport_valid(pass: &String, check_data: bool) -> bool {
    let fields: Vec<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    if check_data {
        return is_passport_data_valid(pass);
    } else {
        return !fields
            .iter()
            .map(|f| pass.contains(f))
            .collect::<Vec<bool>>()
            .contains(&false);
    }
}

fn is_passport_data_valid(pass: &String) -> bool {
    let regexps = vec![
        Regex::new(r"byr:(19[2-9]\d|200[0-2])\s").unwrap(),
        Regex::new(r"iyr:(201\d|2020)\s").unwrap(),
        Regex::new(r"eyr:(202\d|2030)\s").unwrap(),
        Regex::new(r"hgt:(((1[5-8]\d)|19[0-3])cm)|hgt:((59|6\d|7[0-6])in)\s").unwrap(),
        Regex::new(r"hcl:#[0-9a-f]{6}\s").unwrap(),
        Regex::new(r"ecl:(amb|blu|brn|gry|grn|hzl|oth)\s").unwrap(),
        Regex::new(r"pid:\d{9}\s").unwrap(),
        Regex::new(r"^.*:.*:.*:.*:.*:.*:.*:.*|.*:.*:.*:.*:.*:.*:.*:.*:.*$").unwrap(),
    ];

    let mut valid = true;
    for regexp in regexps.iter() {
        valid &= regexp.is_match(pass);
    }
    return valid;
}

fn transform_passports(passports: &[String]) -> Vec<String> {
    let mut idx = 0;
    let mut result = Vec::new();
    loop {
        let mut passport = "".to_owned();
        while !passports[idx].is_empty() {
            passport = passport + &passports[idx] + " ";
            idx += 1;
            if idx >= passports.len() {
                break;
            }
        }
        result.push(passport);

        if idx >= passports.len() {
            break;
        } else {
            idx += 1;
        }
    }

    result
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
