use proconio::input;
fn main() {
    input! {
        n: usize,
    }
    let mut ans = n;
    while ans % 2 == 0 {
        ans /= 2;
    }
    while ans % 3 == 0 {
        ans /= 3;
    }
    println!("{}", if ans == 1 { "Yes" } else { "No" });
}
