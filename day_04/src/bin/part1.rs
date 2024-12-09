fn main() {
    let inp = "input1.txt";
    let out = "hello";
    println!("out: {out}")
}

fn part1(s: &str) -> usize {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn works() {
        let inp = include_str!("../../test1.txt");
        let out = part1(inp);
        assert_eq!(out, 18)
    }
}
