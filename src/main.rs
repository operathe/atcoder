use proconio::input;

fn main() {
    input! {
        a: usize,
        op: String,
        b: usize,
    }
    let ans = if op == "+" {
        a + b
    } else if op == "-" {
        a - b
    } else if op == "*" {
        a * b
    } else if op == "/" {
        a / b
    } else {
        unreachable!();
    };
    // ansを出力する
    println!("{}", ans);
}
