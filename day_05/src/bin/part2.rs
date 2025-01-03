use std::{cmp::Ordering, collections::HashMap};
#[derive(Debug, PartialEq, Eq)]
enum OrderRules {
    Before,
    After,
    Idk,
}

fn main() {
    let inp = include_str!("../../input1.txt");
    let out = part2(inp);
    println!("out: {out}")
}

fn part1(data: &str) -> i32 {
    let first: Vec<&str> = data.split("\n\n").collect();
    let rules = parse_rules(first[0]);
    let mut mid = 0;
    for line in first[1].lines() {
        let print = parse_print(line);
        if check_print(&print, &rules) {
            mid += get_mid(line);
        }
    }
    mid
}

fn part2(data: &str) -> i32 {
    let first: Vec<&str> = data.split("\n\n").collect();
    let rules = parse_rules(first[0]);

    let mut mid = 0;

    for line in first[1].lines() {
        let print = parse_print(line);
        if !check_print(&print, &rules) {
            let fix = fix_print(line, &rules);
            mid += get_mid(fix.as_str());
        }
    }
    mid
}

fn parse_rules(data: &str) -> HashMap<i32, HashMap<i32, OrderRules>> {
    let mut rules: HashMap<i32, HashMap<i32, OrderRules>> = HashMap::new();
    for line in data.lines() {
        let split: Vec<&str> = line.split('|').collect();
        let first = split[0].parse().unwrap();
        let last = split[1].parse().unwrap();

        rules
            .entry(first)
            .or_default()
            .insert(last, OrderRules::After);
        rules
            .entry(last)
            .or_default()
            .insert(first, OrderRules::Before);
    }
    rules
}

fn parse_print(data: &str) -> HashMap<i32, HashMap<i32, OrderRules>> {
    let mut local_map: HashMap<i32, HashMap<i32, OrderRules>> = HashMap::new();
    let nums = parse_to_nums(data);

    for (curr_idx, num) in nums.iter().enumerate() {
        for (other_idx, _) in nums.iter().enumerate() {
            match curr_idx.cmp(&other_idx) {
                Ordering::Less => {
                    local_map
                        .entry(*num)
                        .or_default()
                        .insert(nums[other_idx], OrderRules::After);
                }
                Ordering::Equal => {
                    continue;
                }
                Ordering::Greater => {
                    local_map
                        .entry(*num)
                        .or_default()
                        .insert(nums[other_idx], OrderRules::Before);
                }
            }
        }
    }

    local_map
}

fn parse_to_nums(data: &str) -> Vec<i32> {
    data.split(',')
        .map(|num| -> i32 { num.parse().unwrap() })
        .collect()
}

fn get_mid(data: &str) -> i32 {
    let nums = parse_to_nums(data);
    nums[(nums.len() - 1) / 2]
}

fn check_print(
    print: &HashMap<i32, HashMap<i32, OrderRules>>,
    rules: &HashMap<i32, HashMap<i32, OrderRules>>,
) -> bool {
    let empty: HashMap<i32, OrderRules> = HashMap::new();
    for (num, rule) in print.iter() {
        for (other_idx, order) in rule.iter() {
            let curr_order = rules
                .get(num)
                .unwrap_or(&empty)
                .get(other_idx)
                .unwrap_or(&OrderRules::Idk);
            if curr_order != &OrderRules::Idk && curr_order != order {
                return false;
            }
        }
    }
    true
}

fn fix_print(data: &str, rules: &HashMap<i32, HashMap<i32, OrderRules>>) -> String {
    let nums = parse_to_nums(data);
    let empty: HashMap<i32, OrderRules> = HashMap::new();
    let mut wei: HashMap<i32, Vec<&OrderRules>> = HashMap::new();

    for num in nums.clone() {
        wei.insert(num, Vec::new());
        for num2 in nums.clone() {
            if num == num2 {
                continue;
            }

            let ro = rules
                .get(&num)
                .unwrap_or(&empty)
                .get(&num2)
                .unwrap_or(&OrderRules::Idk);

            wei.get_mut(&num).unwrap().push(ro);
        }
    }

    let mut orders: HashMap<i32, i32> = HashMap::new();

    for (num, list_wei) in wei {
        let mut acc = 0;
        for order in list_wei {
            match order {
                OrderRules::Before => acc += 1,
                OrderRules::After => {
                    acc -= 1;
                }
                OrderRules::Idk => {}
            }
        }

        orders.insert(acc, num);
    }

    let mut key = orders.keys().collect::<Vec<&i32>>();

    key.sort();

    let mut sorted: Vec<&i32> = Vec::new();

    for k in key {
        sorted.push(orders.get(k).unwrap());
    }

    let out = sorted
        .iter()
        .map(|x| format!("{}", x))
        .collect::<Vec<String>>()
        .join(",");

    out
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn works() {
        let inp = include_str!("../../test1.txt");
        let out = part2(inp);
        assert_eq!(out, 123)
    }
}
