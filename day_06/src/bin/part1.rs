fn main() {
    let inp = include_str!("../../input1.txt");
    let out = part1(inp);
    println!("out: {out}")
}

fn part1(data: &str) -> i32 {
    0
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn works() {
        let inp = include_str!("../../test1.txt");
        let out = part1(inp);
        assert_eq!(out, 41)
    }
}
