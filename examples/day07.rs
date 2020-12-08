use std::collections::{HashSet, HashMap};
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

fn construct_map(inputs: Vec<String>) -> HashMap<String, HashSet<(String, usize)>> {
    unimplemented!()
}

fn main() {
    let lines = read_input("day07.txt").unwrap();

    let q1: usize = construct_map(lines)
        .iter()
        .filter(|(&k, &v)| {})
        .count();
    println!("result of q1 is {}", q1);

}
