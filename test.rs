use proconio::input;

fn main() {
    input! {
        a: usize,
        op: String,
        b: usize,
    }
    // bが0の場合はエラーを出力する
    match op {
        ref x if x == "+" => println!("{}", a + b),
        ref x if x == "-" => println!("{}", a - b),
        ref x if x == "*" => println!("{}", a * b),
        ref x if x == "/" => {
            if b == 0 {
                println!("error");
            } else {
                println!("{}", a / b);
            }
        }
        _ => println!("error"),
    }
}
