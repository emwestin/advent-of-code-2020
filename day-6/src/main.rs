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
    let all_groups = get_groups_answers(input);
    let mut counts = Vec::new();
    for (mut group, _) in all_groups {
        group.sort();
        group.dedup();
        counts.push(group.len());
    }
    println!("{}", counts.iter().sum::<usize>());
}

fn part_two(input: &[String]) {
    let all = get_groups_answers(input);
    let mut unique = get_groups_answers(input);
    let mut counts = Vec::new();
    for i in 0..unique.len() {
        unique[i].0.sort();
        unique[i].0.dedup();
        let max = unique[i]
            .0
            .iter()
            .map(|a| all[i].0.iter().filter(|b| b == &a).count())
            .filter(|m| *m == unique[i].1)
            .count();
        counts.push(max);
    }
    println!("{}", counts.iter().sum::<usize>())
}

fn get_groups_answers(input: &[String]) -> Vec<(Vec<char>, usize)> {
    let mut result = Vec::new();
    let mut answers = String::new();
    let mut persons = 0;
    for i in input {
        if i.is_empty() {
            result.push((answers.chars().collect::<Vec<char>>(), persons));
            answers = String::new();
            persons = 0;
        } else {
            answers = answers + i;
            persons += 1;
        }
    }
    result.push((answers.chars().collect::<Vec<char>>(), persons));
    return result;
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
