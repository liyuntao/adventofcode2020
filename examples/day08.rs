use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};


fn read_input(file_name: &str) -> Result<Vec<String>, Error> {
    let path = format!("./input/{}", file_name);
    let br = BufReader::new(File::open(path)?);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn parse_ins(ins_str: &str) -> (&str, i32) {
    let ins_arr: Vec<&str> = ins_str.split(' ').collect();
    let ins: &str = ins_arr.get(0).unwrap();
    let num: i32 = ins_arr.get(1).unwrap().parse::<i32>().unwrap();
    (ins, num)
}

fn ins_executor_q1(ins_list: &Vec<(&str, i32)>) {
    let mut val = 0;
    let mut pointer: usize = 0;
    let mut visited_flag: Vec<bool> = vec![false; ins_list.len()];

    loop {
        // println!("current pointer={}, val={}", pointer, val);
        // pre check
        if visited_flag[pointer] {
            println!("result of q1 is {}", val);
            break;
        }
        visited_flag[pointer] = true; // mark visited
        let tp = ins_list.get(pointer).unwrap();
        match tp.0 {
            "acc" => {
                val += tp.1;
                pointer += 1
            }
            "jmp" => { pointer = ((pointer as i32) + tp.1) as usize }
            "nop" => { pointer += 1 }
            _ => {}
        }
    }
}

fn ins_executor_q2(ins_list: &Vec<(&str, i32)>, try_idx: usize) -> bool {
    if ins_list.get(try_idx).unwrap().0 == "nop" {
        return false;
    }

    let mut val = 0;
    let mut pointer: usize = 0;
    let mut visited_flag: Vec<bool> = vec![false; ins_list.len()];

    while pointer < ins_list.len() {
        // println!("current pointer={}, val={}", pointer, val);
        // pre check
        if visited_flag[pointer] {
            return false;
        }
        visited_flag[pointer] = true; // mark visited
        let tp = ins_list.get(pointer).unwrap();
        let ins_filtered = if pointer == try_idx {
            if tp.0 == "nop" { "jmp" } else { "nop" }
        } else { tp.0 };
        match ins_filtered {
            "acc" => {
                val += tp.1;
                pointer += 1
            }
            "jmp" => { pointer = ((pointer as i32) + tp.1) as usize }
            "nop" => { pointer += 1 }
            _ => {}
        }
    }
    println!("result of q2 is {}", val);
    return true;
}

fn main() {
    let lines = read_input("day08.txt").unwrap();

    let ins_list: Vec<(&str, i32)> = lines.iter()
        .map(|line| parse_ins(line))
        .collect();

    ins_executor_q1(&ins_list);

    for i in 0..ins_list.len() {
        ins_executor_q2(&ins_list, i);
    }
}
