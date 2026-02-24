fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        k: usize,
        ab: [(i64, usize); n],
    }

    let mut dp = vec![vec![i64::MIN; m + 1]; n];
    for (i, &(ai, bi)) in ab.iter().enumerate() {
        dp[i][bi] = ai;
        for j in i.saturating_sub(k)..i {
            for x in 0..=m-bi {
                dp[i][x+bi] = dp[i][x+bi].max(dp[j][x] + ai);
            }
        }
    }
    let mut ans = i64::MIN;
    dp.iter().for_each(|dpi| ans = ans.max(dpi.iter().copied().max().unwrap()));
    println!("{ans}");
}