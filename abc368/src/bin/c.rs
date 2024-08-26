#![allow(unused_imports, unused_variables, dead_code, non_snake_case)]
use proconio::{fastout, input};

#[fastout]
fn main() {
    input! {
        n: usize,
        mut h: [usize; n],
    }

    let mut answer = 0;

    for mut a in h {
        let number = a / 5;
        answer += number * 3;
        a %= 5;

        while a > 0 {
            answer += 1;
            if answer % 3 == 0 {
                a -= 3;
            } else {
                a -= 1;
            }
        }
    }

    println!("{}", answer);
}
