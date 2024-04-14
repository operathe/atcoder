#[allow(unused_imports)]
use proconio::{
    input,
    marker::{Bytes, Chars, Usize1},
};

fn main() {
    input! {
        h: usize,
        w: usize,
        _n: usize,
        t: Bytes,
        s: [Bytes; h],
    }
    let mut res = 0;
    for x in 1..h - 1 {
        for y in 1..w - 1 {
            let mut x = x;
            let mut y = y;
            if s[x][y] == b'#' {
                continue;
            }
            let mut f = true;
            for &c in &t {
                match c {
                    b'U' => x -= 1,
                    b'D' => x += 1,
                    b'L' => y -= 1,
                    b'R' => y += 1,
                    _ => unreachable!(),
                }
                if s[x][y] == b'#' {
                    f = false;
                    break;
                }
            }
            if f {
                res += 1;
            }
        }
    }
    println!("{}", res);
}
