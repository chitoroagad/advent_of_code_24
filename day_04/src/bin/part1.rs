fn main() {
    let inp = include_str!("../../input1.txt");
    let out = part1(inp);
    println!("out: {out}")
}

fn part1(s: &str) -> usize {
    let mat = convert_to_matrix(s);
    let mut total = 0;
    for i in 0..mat.len() {
        for j in 0..mat[0].len() {
            total += exists(i as isize, j as isize, &mat);
        }
    }
    total
}

fn convert_to_matrix(s: &str) -> Vec<Vec<char>> {
    let mut out: Vec<Vec<char>> = vec![];
    for line in s.lines() {
        let chars: Vec<char> = line.chars().collect();
        out.push(chars);
    }
    out
}

fn exists(i: isize, j: isize, grid: &Vec<Vec<char>>) -> usize {
    let dirs: Vec<(isize, isize)> = Vec::from([
        (0, 1),
        (0, -1),
        (1, 0),
        (-1, 0),
        (1, 1),
        (-1, 1),
        (1, -1),
        (-1, -1),
    ]);
    let mut total = 0;
    let (m, n) = (0..grid.len() as isize, 0..grid[0].len() as isize);
    'dirs: for (x, y) in dirs {
        let mut curr = String::new();
        let (mut di, mut dj) = (i, j);
        for _ in 0..4 {
            if !m.contains(&di) || !n.contains(&dj) {
                continue 'dirs;
            }
            curr.push(grid[di as usize][dj as usize]);
            di += x;
            dj += y;
        }
        if curr == "XMAS" {
            total += 1;
        }
    }
    total
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
