use itertools::*;
#[allow(unused_imports)]
use proconio::{input, marker::*};
use std::collections::*;
type Map<K, V> = BTreeMap<K, V>;
type Set<T> = BTreeSet<T>;
type Deque<T> = VecDeque<T>;
fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        t: Bytes,
        s: [Bytes; h],
    }
    let test = |x: usize, y: usize| -> bool {
        if s[x][y] == b'#' {
            return false;
        }
        let dir = [(1, 0), (0, 1), (!0, 0), (0, !0)];
        let p = "DRUL";
        let mut pos = (x, y);
        for &t in t.iter() {
            let k = p.bytes().position(|c| c == t).unwrap();
            pos.0 += dir[k].0;
            pos.1 += dir[k].1;
            if s[pos.0][pos.1] == b'#' {
                return false;
            }
        }
        true
    };
    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if test(i, j) {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}

//     let mut res = 0;
//     for x in 1..h - 1 {
//         for y in 1..w - 1 {
//             let mut x = x;
//             let mut y = y;
//             if s[x][y] == b'#' {
//                 continue;
//             }
//             let mut f = true;
//             for &c in &t {
//                 match c {
//                     b'U' => x -= 1,
//                     b'D' => x += 1,
//                     b'L' => y -= 1,
//                     b'R' => y += 1,
//                     _ => unreachable!(),
//                 }
//                 if s[x][y] == b'#' {
//                     f = false;
//                     break;
//                 }
//             }
//             if f {
//                 res += 1;
//             }
//         }
//     }
//     println!("{}", res);
// }
