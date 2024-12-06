use std::env;
use std::fs::read_to_string;
use std::path::PathBuf;

fn main() {
    let inp = "input1.txt";
    let out = part1(inp);
    println!("out: {out}")
}

fn part1(input: &str) -> u64 {
    let mut total = 0;
    let mut list1: Vec<u64> = Vec::new();
    let mut list2: Vec<u64> = Vec::new();
    let lines: Vec<String> = read_to_string(input)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    for line in lines {
        let split: Vec<u64> = line // get nums in line
            .split_whitespace()
            .map(|x| {
                x.parse::<u64>()
                    .map_err(|_| "Failed to parse".to_string())
                    .unwrap()
            })
            .collect();

        dbg!(split);
    }
    total
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn works() {
        let inp = "test1.txt";
        let out = part1(inp);
        assert_eq!(out, 2)
    }
}
