#[allow(unused_imports)]
use proconio::{input, marker};

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        mut ab: [(usize, usize); n],
    }

    ab.sort_unstable_by_key(|x| x.0);

    let mut dp = [usize::MAX / 2; 3];
    dp[0] = 0;
    let mut i = 0;

    let mut all_left = 0;
    while i < n {
        let ai = ab[i].0;
        let mut min = usize::MAX;
        let mut max = 0;
        let mut max2 = 0;
        while i < n && ab[i].0 == ai {
            max2 = max2.max(ab[i].1);
            if ab[i].1 == w {
                i += 1;
                continue;
            }
            if ab[i].1 == 1 {
                i += 1;
                continue;
            }
            min = min.min(ab[i].1);
            max = max.max(ab[i].1);
            i += 1;
        }
        all_left += 2 * (max2 - 1);
        if min == usize::MAX {
            continue;
        }
        let mut ndp = [usize::MAX; 3];
        ndp[2] = dp[0] + w - 1;
        ndp[0] = dp[0] + 2 * (max - 1);
        ndp[1] = ndp[1].min(dp[1] + 2 * (max - 1));
        ndp[1] = ndp[1].min(dp[2] + w - 1);
        ndp[2] = ndp[2].min(dp[2] + 2 * (h - min));
        ndp[2] = ndp[2].min(dp[1] + w - 1);
        dp = ndp;
    }

    let ans = dp[1].min(all_left);
    println!("{}", ans);
}
