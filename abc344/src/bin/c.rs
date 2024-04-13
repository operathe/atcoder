use proconio::{fastout, input};
use std::collections::HashSet;
#[fastout]
fn main() {
    input! {
        n: usize,
        a: [isize; n],
        m: usize,
        b: [isize; m],
        l: usize,
        c: [isize; l],
        q: usize,
        x: [isize; q],
    }

    let ab_sums: HashSet<_> = a
        .iter()
        .flat_map(|&ai| b.iter().map(move |&bi| ai + bi))
        .collect();

    for &xi in &x {
        let found = c.iter().any(|&ci| ab_sums.contains(&(xi - ci)));
        println!("{}", if found { "Yes" } else { "No" });
    }
}
