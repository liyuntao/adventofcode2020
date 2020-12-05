use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Error, ErrorKind};
use regex::Regex;


fn read_input(file_name: &str) -> Result<Vec<String>, Error> {
    let path = format!("./input/{}", file_name);
    let br = BufReader::new(File::open(path)?);
    br.lines()
        .map(|line| line.and_then(|v| v.parse().map_err(|e| Error::new(ErrorKind::InvalidData, e))))
        .collect()
}

fn group(input: Vec<String>) -> Vec<HashMap<String, String>> {
    let mut res: Vec<HashMap<String, String>> = Vec::new();

    let mut passport: HashMap<String, String> = HashMap::new();
    for line in input {
        if line.is_empty() {
            res.push(passport);
            passport = HashMap::new();
        } else {
            parse_and_merge(&mut passport, line);
        }
    }
    res.push(passport);
    res
}

fn parse_and_merge(map: &mut HashMap<String, String>, line: String) {
    line.split(' ').for_each(|part| {
        let key_value: Vec<&str> = part.split(':').collect();
        map.insert(key_value[0].to_string(), key_value[1].to_string());
    });
}

fn is_valid_passport_q1(passport: &HashMap<String, String>) -> bool {
    // for (key, value) in passport.into_iter() {
    //     println!("{} / {}", key, value);
    // }

    passport.contains_key("byr")
        && passport.contains_key("iyr")
        && passport.contains_key("eyr")
        && passport.contains_key("hgt")
        && passport.contains_key("hcl")
        && passport.contains_key("ecl")
        && passport.contains_key("pid")
}

fn is_valid_passport_q2(passport: &HashMap<String, String>) -> bool {
    is_byr_valid(passport)
        && is_iyr_valid(passport)
        && is_iyr_valid(passport)
        && is_eyr_valid(passport)
        && is_hgt_valid(passport)
        && is_hcl_valid(passport)
        && is_ecl_valid(passport)
        && is_pid_valid(passport)
}

fn is_byr_valid(passport: &HashMap<String, String>) -> bool {
    let byr = passport.get("byr").unwrap();
    byr.len() == 4
        && match byr.parse::<i32>() {
            Ok(x) => x >= 1920 && x <= 2002,
            Err(_) => false,
        }
}

fn is_iyr_valid(passport: &HashMap<String, String>) -> bool {
    let iyr = passport.get("iyr").unwrap();
    iyr.len() == 4
        && match iyr.parse::<i32>() {
            Ok(x) => x >= 2010 && x <= 2020,
            Err(_) => false,
        }
}

fn is_eyr_valid(passport: &HashMap<String, String>) -> bool {
    let eyr = passport.get("eyr").unwrap();
    eyr.len() == 4
        && match eyr.parse::<i32>() {
            Ok(x) => x >= 2020 && x <= 2030,
            Err(_) => false,
        }
}

fn is_hgt_valid(passport: &HashMap<String, String>) -> bool {
    let hgt = passport.get("hgt").unwrap();
    if hgt.ends_with("cm") && hgt.len() == 5 {
        let no: String = hgt.chars().take(3).collect();
        match no.parse::<usize>() {
            Ok(x) => x >= 150 && x <= 193,
            Err(_) => false,
        }
    } else if hgt.ends_with("in") && hgt.len() == 4 {
        let no: String = hgt.chars().take(2).collect();
        match no.parse::<usize>() {
            Ok(x) => x >= 59 && x <= 76,
            Err(_) => false,
        }
    } else {
        false
    }
}

fn is_hcl_valid(passport: &HashMap<String, String>) -> bool {
    let hcl = passport.get("hcl").unwrap();
    let re = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    re.is_match(hcl)
}
fn is_ecl_valid(passport: &HashMap<String, String>) -> bool {
    let ecl = passport.get("ecl").unwrap();
    ecl == "amb"
        || ecl == "blu"
        || ecl == "brn"
        || ecl == "gry"
        || ecl == "grn"
        || ecl == "hzl"
        || ecl == "oth"
}

fn is_pid_valid(passport: &HashMap<String, String>) -> bool {
    let pid = passport.get("pid").unwrap();
    pid.len() == 9
        && match pid.parse::<i32>() {
            Ok(_) => true,
            Err(_) => false,
        }
}

fn main() {
    let lines = read_input("day04.txt").unwrap();
    let passport_list = group(lines);

    let q1 = passport_list
        .iter()
        .filter(|&passport| is_valid_passport_q1(passport))
        .count();
    println!("result of q01 is {}", q1);

    let q2 = passport_list
        .iter()
        .filter(|&passport| is_valid_passport_q1(passport))
        .filter(|&passport| is_valid_passport_q2(passport))
        .count();
    println!("result of q01 is {}", q2);
}
