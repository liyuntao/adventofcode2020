use std::collections::{HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};


fn read_input(file_name: &str) -> Result<Vec<String>, Error> {
    let path = format!("./input/{}", file_name);
    let br = BufReader::new(File::open(path)?);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn group_q1(input: &Vec<String>) -> Vec<HashSet<char>> {
    let mut res: Vec<HashSet<char>> = Vec::new();

    let mut question_set: HashSet<char> = HashSet::new();
    for line in input {
        if line.is_empty() {
            res.push(question_set);
            question_set = HashSet::new();
        } else {
            line.chars().for_each(|c| {
                question_set.insert(c);
            });
        }
    }
    res.push(question_set);
    res
}

fn group_q2(input: &Vec<String>) -> Vec<HashSet<char>> {
    let mut res: Vec<HashSet<char>> = Vec::new();

    let mut question_set: HashSet<char> = new_init_set();
    for line in input {
        if line.is_empty() {
            res.push(question_set);
            question_set = new_init_set();
        } else {
            let set2: HashSet<char> = line.chars().collect();
            question_set = question_set.intersection(&set2).copied().collect();
        }
    }
    res.push(question_set);
    res
}

fn new_init_set() -> HashSet<char> {
    let mut set: HashSet<char> = HashSet::new();
    ('a'..='z').for_each(|c| { set.insert(c); });
    set
}

fn main() {
    let lines = read_input("day06.txt").unwrap();

    let q1: usize = group_q1(&lines)
        .iter()
        .map(|set| set.len())
        .sum();
    println!("result of q1 is {}", q1);

    let q2: usize = group_q2(&lines)
        .iter()
        .map(|set| set.len())
        .sum();
    println!("result of q2 is {}", q2);
}
