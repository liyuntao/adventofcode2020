use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use std::collections::HashMap;

fn gen_grid() -> (usize, usize, HashMap<(usize, usize), bool /*isTree*/>) {
    let path = format!("./input/{}", "day03.txt");
    let lines: Vec<String> = BufReader::new(File::open(path).unwrap())
        .lines()
        .map(|l_ei| l_ei.expect("Could not parse line"))
        .collect();

    let mut map: HashMap<(usize, usize), bool> = HashMap::new();
    for (j, row) in lines.iter().enumerate() {
        for (i, c) in row.chars().enumerate() {
            map.insert((i, j), c == '#');
        }
    }
    (lines[0].len(), lines.len(), map)
}

fn q1(col_count: usize, row_count: usize, data_set: &HashMap<(usize, usize), bool>) -> usize {
    println!("start: {} {}", col_count, row_count);
    let mut res: usize = 0;
    let mut i: usize = 0;
    let mut j: usize = 0;
    while j < row_count {
        println!("{} {}", i, j);
        if *data_set.get(&(i, j)).unwrap() {
            res += 1;
        }
        i = (i + 3) % col_count;
        j +=1;
    }
    res
}

fn main() {
    let (col_count, row_count, grid) = gen_grid();
    let q1_res = q1(col_count, row_count, &grid);
    println!("result of q1 is {}", q1_res);

    // let q2_res: usize = lines.iter().filter(|l| jduge_q2(l)).count();
    // println!("result of q02 is {}", q2_res);
}
