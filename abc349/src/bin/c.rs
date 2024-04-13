use proconio::{input, marker::Chars};

fn main() {
    input! {
        s:Chars,
        mut t:Chars,
    }
    let mut i = 0;
    for c in &s {
        if i >= t.len() {
            break;
        }
        if c.to_ascii_uppercase() == t[i] {
            i += 1;
        }
    }
    if i == 3 || (i == 2 && t[2] == 'X') {
        println!("Yes");
    } else {
        println!("No");
    }
}
