use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }
    let s = s.into_iter().collect::<Vec<char>>();
    //１文字目が＜、最後の文字が＞の場合
    if s[0] == '<' && s[s.len() - 1] == '>' {
        if s[1..s.len() - 1].iter().all(|&c| c == '=') {
            println!("Yes");
        } else {
            println!("No");
        }
    } else {
        println!("No");
    }
}
