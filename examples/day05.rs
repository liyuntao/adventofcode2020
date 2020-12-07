use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};

pub fn read_input(file_name: &str) -> Result<Vec<String>, Error> {
    let path = format!("./input/{}", file_name);
    let br = BufReader::new(File::open(path)?);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn cal_seat_id(cmd: &str) -> i32 {
    let mut r_low: i32 = 0;
    let mut r_high: i32 = 127;
    let mut c_low: i32 = 0;
    let mut c_high: i32 = 7;
    cmd.chars().for_each(|c| {
        match c {
            'F' => {
                r_high = r_low + (r_high - r_low) / 2
            }
            'B' => {
                r_low = r_low + (r_high - r_low) / 2 + 1
            }
            'L' => {
                c_high = c_low + (c_high - c_low) / 2
            }
            'R' => {
                c_low = c_low + (c_high - c_low) / 2 + 1
            }
            _ => {}
        }
        // println!("{} {} {} {}", r_low, r_high, c_low, c_high)
    });
    r_low * 8 + c_low
}

fn main() {
    let lines = read_input("day05.txt").unwrap();
    let q1 = lines.iter()
        .map(|ins| cal_seat_id(ins))
        .max()
        .unwrap();
    println!("result of q1 is {}", q1);

    let lines2 = read_input("day05.txt").unwrap();
    let vec: Vec<i32> = lines2.iter()
        .map(|ins| cal_seat_id(ins))
        .collect();
    let min = *vec.iter().min().unwrap();
    let max = *vec.iter().max().unwrap();
    let mut id_sum: i32 = (min..=max).sum();
    vec.iter()
        .for_each(|id| { id_sum = id_sum - id });
    println!("result of q2 is {}", id_sum);
}
