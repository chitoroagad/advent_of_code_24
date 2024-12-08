use regex::Regex;
use std::fs::read_to_string;
fn main() {
    let inp = "input1.txt";
    let out = part1(inp);
    println!("out: {out}")
}

fn parse_inp(input_path: &str) -> Vec<String> {
    let inp = read_to_string(input_path);
    inp.unwrap().lines().map(String::from).collect()
}

fn get_vals(s: String) -> Vec<i64> {
    let pattern = r"mul\((\d+),(\d+)\)";
    let re = Regex::new(pattern).unwrap();
    let mut results: Vec<(i64, i64)> = Vec::new();
    for (_, [first, second]) in re.captures_iter(s.as_str()).map(|caps| caps.extract()) {
        results.push((first.parse().unwrap(), second.parse().unwrap()))
    }
    dbg!(&results);
    results.iter().map(|(x, y)| x * y).collect::<Vec<i64>>()
}

fn part1(input: &str) -> i64 {
    let mut total = 0;
    let lines = parse_inp(input);
    for line in lines {
        total += get_vals(line).iter().sum::<i64>();
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
        assert_eq!(out, 161)
    }
}
