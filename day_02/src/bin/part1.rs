use std::env;
use std::fs::read_to_string;
use std::path::PathBuf;

fn main() {
    let inp = "input1.txt";
    let out = part1(inp);
    println!("out: {out}")
}

fn part1(input: &str) -> i64 {
    let mut total = 0;
    let mut list1: Vec<i64> = Vec::new();
    let mut list2: Vec<i64> = Vec::new();
    let lines: Vec<String> = read_to_string(input)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    'outer: for line in lines {
        let split: Vec<i64> = line // get nums in line
            .split_whitespace()
            .map(|x| {
                x.parse::<i64>()
                    .map_err(|_| "Failed to parse".to_string())
                    .unwrap()
            })
            .collect();

        let decreasing = split[0] > split[1];
        for i in 1..split.len() {
            let abs_diff = split[i].abs_diff(split[i - 1]);

            if !(1..=3).contains(&abs_diff) {
                continue 'outer;
            }
            if decreasing && split[i] > split[i - 1] {
                continue 'outer;
            }
            if !decreasing && split[i] < split[i - 1] {
                continue 'outer;
            }
        }
        total += 1;
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
