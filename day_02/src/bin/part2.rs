use std::fs::read_to_string;
fn main() {
    let inp = "input1.txt";
    let out = part2(inp);
    println!("out: {out}")
}

fn part1(input: &str) -> i64 {
    let mut total = 0;
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

fn part2(input: &str) -> i64 {
    let txt = read_to_string(input).unwrap();
    let reports = parse_inp(txt);
    reports.iter().filter(|rep| is_safe(rep, true)).count() as i64
}

fn parse_inp(input: String) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|report| {
            report
                .split_whitespace()
                .map(|level| {
                    dbg!(level);
                    level.parse::<i64>().unwrap()
                })
                .collect()
        })
        .collect()
}

fn is_safe(report: &Vec<i64>, try_remove: bool) -> bool {
    #[derive(PartialEq)]
    enum Order {
        Asc,
        Desc,
    };
    let exp_climb = match report[0].cmp(&report[1]) {
        std::cmp::Ordering::Less => Order::Asc,
        std::cmp::Ordering::Greater => Order::Desc,
        std::cmp::Ordering::Equal => return can_be_made_safe(report, try_remove),
    };
    let exp_range = 1..=3;
    for i in 0..report.len() - 1 {
        let l = report[i];
        let r = report[i + 1];

        let climb = match l.cmp(&r) {
            std::cmp::Ordering::Less => Order::Asc,
            std::cmp::Ordering::Greater => Order::Desc,
            std::cmp::Ordering::Equal => return can_be_made_safe(report, try_remove),
        };
        if climb != exp_climb {
            return can_be_made_safe(report, try_remove);
        }
        let range = l.abs_diff(r);
        if !exp_range.contains(&range) {
            return can_be_made_safe(report, try_remove);
        }
    }
    true
}

fn can_be_made_safe(report: &Vec<i64>, try_remove: bool) -> bool {
    if try_remove {
        for i in 0..report.len() {
            let mut left_removed = report.clone();
            left_removed.remove(i);
            if is_safe(&left_removed, false) {
                return true;
            }
        }
    }
    false
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

    #[test]
    fn works2() {
        let inp = "test1.txt";
        let out = part2(inp);
        assert_eq!(out, 4)
    }
}
