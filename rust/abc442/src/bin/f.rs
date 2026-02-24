use proconio::{input, marker};
fn main() {
    input! {
        n: usize,
        s: [marker::Bytes; n],
    }
    let mut dp = vec![0u32; n + 1];
    // let mut ndp = dp.clone();
    for si in &s {
        for i in (0..n).rev() {
            dp[i] = dp[i].min(dp[i + 1]);
        }
        let mut cnt2 = si.iter().filter(|&&sij| sij == b'.').count() as u32;
        // let mut cnt = 0;
        dp[0] += cnt2;
        for i in 0..n {
            if si[i] == b'.' {
                cnt2 -= 1;
            } else {
                cnt2 += 1;
            }
            dp[i + 1] += cnt2;
        }
    }
    let ans = dp.iter().copied().min().unwrap();
    println!("{ans}");
}