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
    let ids = get_ids(input);
    let result = ids.iter().max().unwrap();
    println!("{}", result);
}

fn part_two(input: &[String]) {
    let mut rows = get_ids(input);
    rows.sort();
    for i in 0..rows.len() - 2 {
        if rows[i] + 1 != rows[i + 1] && rows[i] + 2 == rows[i + 2] {
            println!("{}", rows[i] + 1);
            return;
        }
    }
}

fn get_ids(encoded_seats: &[String]) -> Vec<usize> {
    return encoded_seats
        .iter()
        .map(|s| {
            (
                decode(&s[0..=6], 0, 128, 'F', 'B'),
                decode(&s[7..=9], 0, 7, 'L', 'R'),
            )
        })
        .map(|r| r.0 * 8 + r.1)
        .collect::<Vec<usize>>();
}

fn decode(encoded_seat: &str, low: usize, high: usize, lower: char, upper: char) -> usize {
    let mut l = low;
    let mut h = high;
    for c in encoded_seat.chars() {
        if c == lower {
            h = h - ((h - l) / 2);
        } else if c == upper {
            l = l + ((h - l) / 2);
        }
    }
    return if h > l { l } else { h };
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
