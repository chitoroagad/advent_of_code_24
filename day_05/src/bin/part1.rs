use std::collections::{HashMap, HashSet};
fn main() {
    let inp = include_str!("../../input1.txt");
    let out = part1(inp);
    println!("out: {out}")
}

fn part1(s: &str) -> usize {
    let mut total = 0;
    let data = String::from(s);
    create_dict(s);
    total
}

fn create_dict(s: &str) -> (usize, HashMap<u32, HashSet<u32>>) {
    let mut map: HashMap<u32, HashSet<u32>> = HashMap::new();
    let mut mid = 0;
    for (i, line) in s.lines().enumerate() {
        if line.is_empty() {
            mid = i;
            break;
        }
        let vals: Vec<u32> = line
            .split("|")
            .map(|st| st.parse::<u32>().unwrap())
            .collect();
        let (l, r) = (vals[0], vals[1]);
        let set = map.entry(r).or_default();
        set.insert(l);
    }
    dbg!(&map);
    (mid, map)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn works() {
        let inp = include_str!("../../test1.txt");
        let out = part1(inp);
        assert_eq!(out, 143)
    }
}
