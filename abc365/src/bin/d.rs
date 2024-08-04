use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut dp = vec![[0usize; 3]; n + 1];

    for i in 0..n {
        let c = s[i];
        if c == 'R' {
            dp[i + 1][0] += dp[i][1].max(dp[i][2]);
            dp[i + 1][1] += dp[i][0].max(dp[i][2]) + 1;
        } else if c == 'P' {
            dp[i + 1][1] += dp[i][0].max(dp[i][2]);
            dp[i + 1][2] += dp[i][0].max(dp[i][1]) + 1;
        } else {
            dp[i + 1][2] += dp[i][0].max(dp[i][1]);
            dp[i + 1][0] += dp[i][1].max(dp[i][2]) + 1;
        }
    }

    let ans = dp[n].iter().max().unwrap();
    println!("{}", ans);
}
