#[macro_use]
extern crate lazy_static;
extern crate regex;

use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use regex::Regex;


fn jduge_q1(input: &str) -> bool {
    // 10-16 b: bbbbbbbbbbbbbbbsn
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+)-(\d+)\s([a-z]{1}):\s([a-z]+)").unwrap();
    }

    let caps = RE.captures(input).unwrap();
    let at_least = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
    let at_most = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
    let key_char = caps.get(3).unwrap().as_str().parse::<char>().unwrap();
    let pwd: String = caps.get(4).unwrap().as_str().parse::<String>().unwrap();

    // dbg!("{} {} {} {}", at_least, at_most, key_char, pwd);

    let key_char_count: usize = pwd.chars()
        .filter(|c| *c == key_char)
        .count();

    return (key_char_count >= at_least) && (key_char_count <= at_most);
}

fn jduge_q2(input: &str) -> bool {
    // 10-16 b: bbbbbbbbbbbbbbbsn
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(\d+)-(\d+)\s([a-z]{1}):\s([a-z]+)").unwrap();
    }

    let caps = RE.captures(input).unwrap();
    let at_least = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
    let at_most = caps.get(2).unwrap().as_str().parse::<usize>().unwrap();
    let key_char = caps.get(3).unwrap().as_str().parse::<char>().unwrap();
    let pwd: String = caps.get(4).unwrap().as_str().parse::<String>().unwrap();


    return (pwd.chars().nth(at_least - 1).unwrap() == key_char)
        ^ (pwd.chars().nth(at_most - 1).unwrap() == key_char);
}

fn main() {
    let path = format!("./input/{}", "day02.txt");
    let lines: Vec<String> = BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|l_ei| l_ei.expect("Could not parse line"))
        .collect();

    let q1_res: usize = lines.iter().filter(|l| jduge_q1(l)).count();
    println!("result of q01 is {}", q1_res);


    let q2_res: usize = lines.iter().filter(|l| jduge_q2(l)).count();
    println!("result of q02 is {}", q2_res);
}
