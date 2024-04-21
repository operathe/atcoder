use proconio::input;
use proconio::marker::Chars;

fn main() {
    input! {
        s: Chars,
    }

    let ans = if s[0] == 'A' && s[1] == 'B' && s[2] == 'C' {
        let n = s[3..].iter().collect::<String>().parse::<i64>().unwrap();
        if n == 316 || n >= 350 || n == 0 {
            "No"
        } else {
            "Yes"
        }
    } else {
        "No"
    };

    println!("{}", ans);
}
