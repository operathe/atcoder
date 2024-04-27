use proconio::{fastout, input};
#[allow(non_snake_case)]
#[fastout]
fn main() {
    input! {
        a: usize, b: usize, d:usize,
    }
    let ans: Vec<usize> = (a..=b).step_by(d).collect();
    println!(
        "{}",
        ans.iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
