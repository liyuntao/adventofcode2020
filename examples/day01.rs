use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

pub fn read_input(file_name: &str) -> Result<HashSet<i64>, Error> {
    let path = format!("./input/{}", file_name);
    let br = BufReader::new(File::open(path)?);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

pub fn two_sum(numbers: &HashSet<i64>, exclude: i64, sum_eq: i64) -> i64 {
    for i in numbers.iter() {
        if *i != exclude && numbers.contains(&(sum_eq - i)) {
            return i * (sum_eq - i);
        }
    }
    return 0;
}

fn main() {
    let numbers = read_input("day01.txt").unwrap();
    for i in numbers.iter() {
        if numbers.contains(&(2020 - i)) {
            println!("result of q1 is {}", i * (2020 - i));
            break;
        }
    }

    for i in numbers.iter() {
        let two_sum_res = two_sum(&numbers, i.clone(), 2020 - i);
        if two_sum_res > 0 {
            println!("result of q2 is {}", i * two_sum_res);
            break;
        }
    }
}
