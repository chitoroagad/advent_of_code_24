use std::collections::{HashMap, HashSet};
use std::env;
use std::fs::read_to_string;

fn main() {
    let inp = "input1.txt";
    let current_dir = env::current_dir().unwrap();
    println!("The current directory is: {}", current_dir.display());
    let out = part1(inp);
    let out2 = part2(inp);
    println!("out: {out}");
    println!("out2: {out2}");
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
        let split: Vec<&str> = line.split_whitespace().collect();

        // Parse the two parts as u32
        let num1: u64 = split[0]
            .parse()
            .map_err(|_| "Failed to parse the first number".to_string())
            .unwrap();
        let num2: u64 = split[1]
            .parse()
            .map_err(|_| "Failed to parse the second number".to_string())
            .unwrap();
        list1.push(num1);
        list2.push(num2);
    }
    list1.sort();
    list2.sort();
    for it in list1.iter().zip(list2.iter()) {
        let (a, b) = it;
        total += a.abs_diff(*b);
    }
    total
}

fn part2(input: &str) -> u64 {
    let mut list1: Vec<u64> = Vec::new();
    let mut list2: Vec<u64> = Vec::new();
    let lines: Vec<String> = read_to_string(input)
        .unwrap()
        .lines()
        .map(String::from)
        .collect();
    for line in lines {
        let split: Vec<&str> = line.split_whitespace().collect();

        // Parse the two parts as u32
        let num1: u64 = split[0]
            .parse()
            .map_err(|_| "Failed to parse the first number".to_string())
            .unwrap();
        let num2: u64 = split[1]
            .parse()
            .map_err(|_| "Failed to parse the second number".to_string())
            .unwrap();
        list1.push(num1);
        list2.push(num2);
    }

    let mut dict: HashMap<u64, u64> = HashMap::new();
    let mut set: HashSet<u64> = HashSet::new();

    for el in &list1 {
        set.insert(*el);
    }

    for a in &set {
        for b in &list2 {
            if *a == *b {
                let num = dict.entry(*a).or_insert(0);
                *num += 1;
            }
        }
    }
    let mut out: u64 = 0;
    for n in list1 {
        out += n * dict.get(&n).unwrap_or(&0);
    }
    out
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn works2() {
        let inp = "test1.txt";
        let out = part2(inp);
        assert_eq!(out, 31)
    }

    #[test]
    fn works() {
        let inp = "test1.txt";
        let out = part1(inp);
        assert_eq!(out, 11)
    }
}
