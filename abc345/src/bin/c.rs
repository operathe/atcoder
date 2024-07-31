use proconio::input;
use proconio::marker::*;
fn main() {
    input! {
            s: Bytes,
    }
    let mut count = [0 as u64; 26];
    for c in s {
        count[(c - b'a') as usize] += 1;
    }
    println!("{:?}", count);
    let mut ans = 0;
    for (i, a) in count.iter().enumerate() {
        for b in count.iter().take(i) {
            ans += a * b;
        }
    }
    println!("{}", ans);
    if count.iter().any(|c| *c >= 2) {
        ans += 1;
    }
    println!("{}", ans);
}
//
// fn main() {
//     input! {
//         s: String,
//     }
//     let n = s.len();
//     println!("{}", n);
//     let mut count = [0 as usize; 26];
//     for c in s.chars() {
//         count[c as usize - 'a' as usize] += 1;
//     }
//     println!("{:?}", count);
//     let same = count.iter().map(|&c| c * (c - 1) / 2).sum::<usize>();
//     println!("{}", same);
//     let ans = n * (n - 1) / 2 - if same == 0 { 0 } else { same - 1 };
//     println!("{}", ans);
// }
