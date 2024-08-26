#![allow(unused_imports, unused_variables, dead_code, non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut h: [usize; n],
    }

    let mut T: usize = 0;

    for mut a in h {
        let num = a / 5;
        T += num * 3;
        a %= 5;

        while a > 0 {
            T += 1;
            if T % 3 == 0 {
                if a >= 3 {
                    a -= 3;
                } else {
                    a = 0; // aが3未満の場合、0にする
                }
            } else {
                a -= 1;
            }
        }
    }

    println!("{}", T);
}
