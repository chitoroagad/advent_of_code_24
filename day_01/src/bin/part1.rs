use std::env;
use std::fs::read_to_string;
use std::path::PathBuf;

fn main() {
    let inp = "input1.txt";
    let current_dir = env::current_dir().unwrap();
    println!("The current directory is: {}", current_dir.display());
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn works2() {
        let current_dir = env::current_dir().unwrap();
        println!("The current directory is: {}", current_dir.display());
    }

    #[test]
    fn works() {
        let inp = "test1.txt";
        let out = part1(inp);
        assert_eq!(out, 11)
    }
}
