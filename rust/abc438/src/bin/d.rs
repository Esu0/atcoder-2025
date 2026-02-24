use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u64; n],
        b: [u64; n],
        c: [u64; n],
    }

    let mut asum = a[0];
    let mut dp = vec![0; n];
    dp[1] = a[0];
    for i in 1..n - 1 {
        dp[i + 1] = (asum + b[i]).max(dp[i] + b[i]);
        asum += a[i];
    }

    let mut ans = 0;
    let mut csum = 0;
    for (i, &ci) in c.iter().enumerate().skip(2).rev() {
        csum += ci;
        ans = ans.max(csum + dp[i]);
    }
    println!("{ans}");
}
