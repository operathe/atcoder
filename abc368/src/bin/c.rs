#![allow(unused_imports, unused_variables, dead_code, non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut h: [usize; n],
    }

    let mut ans = 0;

    for mut a in h {
        let tmp = a / 5;
        ans += tmp * 3;
        a %= 5;

        while a > 0 {
            ans += 1;
            if ans % 3 == 0 {
                if a >= 3 {
                    a -= 3;
                } else {
                    a = 0;
                }
            } else {
                a -= 1;
            }
        }
    }

    println!("{}", ans);
}
