use proconio::{input, marker};

fn main() {
    input! {
        n: usize,
        h: usize,
        w: usize,
        s: [[marker::Bytes; h]; n],
    }
    assert_eq!(s[0][0][0], b'0');
    assert_eq!(s[n - 1][h - 1][w - 1], b'0');

    let mut dp = vec![vec![vec![u32::MAX; w]; h]; n];
    dp[0][0][0] = 0;
    for l in 0..n {
        for i in 0..h {
            for j in 0..w {
                if s[l][i][j] == b'H' {
                    dp[l + 1][i][j] = dp[l + 1][i][j].min(dp[l][i][j]);
                } else {
                    let c = (s[l][i][j] - b'0') as u32;
                    if i + 1 < h {
                        dp[l][i + 1][j] = dp[l][i + 1][j].min(dp[l][i][j].saturating_add(c));
                    }
                    if j + 1 < w {
                        dp[l][i][j + 1] = dp[l][i][j + 1].min(dp[l][i][j].saturating_add(c));
                    }
                }
            }
        }
    }
    println!("{}", dp[n - 1][h - 1][w - 1]);
}
