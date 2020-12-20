use regex::Regex;
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

struct Rule {
    outer: String,
    inner: Vec<(usize, String)>,
}

fn part_one(input: &[String]) {
    let rules = get_rules(input);
    let mut can_fit = HashSet::new();
    let mut prev = 9999999;
    while prev != can_fit.len() {
        prev = can_fit.len();
        for i in 0..rules.len() {
            if !can_fit.contains(&rules[i].outer) {
                for j in &rules[i].inner {
                    if j.1 == "shiny gold" || can_fit.contains(&j.1) {
                        can_fit.insert(&rules[i].outer);
                    }
                }
            }
        }
    }
    println!("{:?}", can_fit.len());
}

fn part_two(input: &[String]) {
    let rules = get_rules(input);
    let gold_bag = rules
        .iter()
        .filter(|r| r.outer == "shiny gold")
        .next()
        .unwrap();
    let count = gold_bag
        .inner
        .iter()
        .map(|r| r.0 + r.0 * count_inner_bags(r, &rules))
        .sum::<usize>();

    println!("{:?}", count);
}

fn count_inner_bags(bag: &(usize, String), rules: &Vec<Rule>) -> usize {
    let rule = rules.iter().filter(|r| r.outer == bag.1).next().unwrap();
    return if rule.inner.len() == 0 {
        0
    } else {
        rule.inner
            .iter()
            .map(|r| r.0 + r.0 * count_inner_bags(r, rules))
            .sum::<usize>()
    };
}

fn get_rules(input: &[String]) -> Vec<Rule> {
    let regex = Regex::new(r"([a-z]+\s[a-z]+)|\d").unwrap();
    let mut rules = Vec::new();

    for i in input {
        let captures = regex
            .find_iter(i)
            .map(|f| String::from(f.as_str()))
            .collect::<Vec<String>>();

        let mut to = Vec::new();
        for j in (2..captures.len()).step_by(2) {
            match captures[j].parse::<usize>() {
                Ok(c) => to.push((c, String::from(&captures[j + 1]))),
                _ => {}
            }
        }
        let rule = Rule {
            outer: String::from(&captures[0]),
            inner: to,
        };
        rules.push(rule);
    }

    return rules;
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
