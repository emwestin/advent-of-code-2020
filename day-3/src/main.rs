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
    let map = input.to_vec();
    let result = count_trees(&map, 3, 1);

    println!("{}", result);
}

fn part_two(input: &[String]) {
    let map = input.to_vec();
    let slopes = vec![(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let mut result = 1;

    for slope in slopes.iter() {
        result *= count_trees(&map, slope.0, slope.1);
    }

    println!("{}", result);
}

fn count_trees(map: &Vec<String>, right: usize, down: usize) -> usize {
    let mut result = 0;
    let mut x = 0;
    let width = map[0].len();

    for y in (down..map.len()).step_by(down) {
        x += right;
        if map[y].chars().nth(x % width).unwrap() == '#' {
            result += 1;
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
