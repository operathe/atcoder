use itertools::*;
use proconio::*;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
    }
    let mut s = vec![vec!['.'; w]; h];
    let mut x = 0;
    let mut y = 0;
    let mut d = 0;
    let dir = [(h - 1, 0), (0, 1), (1, 0), (0, w - 1)];
    for _ in 0..n {
        if s[x][y] == '.' {
            s[x][y] = '#';
            d = (d + 1) % 4;
        } else {
            s[x][y] = '.';
            d = (d + 3) % 4;
        }
        let dir = dir[d];
        x = (x + dir.0) % h;
        y = (y + dir.1) % w;
    }
    for s in s {
        println!("{}", s.iter().join(""));
    }
}
