use proconio::{input, marker};

fn main() {
    input! {
        n: usize,
        s: [marker::Bytes; n],
    }

    let mut dp = vec![vec![vec![vec![0; n + 1]; n + 1]; n + 1]; n + 1];
    for h in 1..=n {
        for w in 1..=n {
            for u in 0..=n - h {
                for l in 0..=n - w {
                    let mut v = h.max(w);
                    for i in u..u + h {
                        if s[i][l..][..w].iter().all(|&c| c == b'.') {
                            v = v.min(dp[i - u][w][u][l] + dp[u + h - i - 1][w][i + 1][l]);
                        }
                    }
                    for i in l..l + w {
                        if s[u..][..h].iter().all(|row| row[i] == b'.') {
                            v = v.min(dp[h][i - l][u][l] + dp[h][w + l - i - 1][u][i + 1]);
                        }
                    }
                    dp[h][w][u][l] = v;
                }
            }
        }
    }
    println!("{}", dp[n][n][0][0]);
}
