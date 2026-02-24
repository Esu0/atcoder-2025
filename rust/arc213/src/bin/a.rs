fn main() {
    proconio::input! {
        n: usize,
        l: usize,
        mut cp: [(i32, [u8; l]); n],
    }
    let mut buf = vec![0; l];
    for (_, pi) in &mut cp {
        pi.iter_mut().for_each(|a| *a -= 1);
    }
    for i in 0..l { buf[i] = i as u8; }

    let mut set = vec![false; l];

    let mut mx = i32::MIN;
    let mxinv = l * (l - 1) / 2 + 2;
    let mut dp = vec![i32::MIN; n + 1];
    dp[0] = 0;
    for (i, (ci, pi)) in cp.iter().enumerate() {
        let ci = *ci;
        for p in 1..=mxinv {
            if p > i + 1 {
                continue;
            }
            let mut cnt = 0;
            let pj: &[u8] = if i + 1 == p {
                &buf
            } else {
                &cp[i - p].1
            };
            {
                set.fill(false);
                for &pik in pi {
                    for &pjk in pj {
                        if pik == pjk { break }
                        if !set[pjk as usize] { cnt += 1 }
                    }
                    set[pik as usize] = true;
                }
            }
            if cnt <= p {
                dp[i + 1] = dp[i + 1].max(dp[i + 1 - p] + ci);
            }
        }
        if i + 1 >= mxinv {
            mx = mx.max(dp[i + 1 - mxinv]);
            dp[i + 1] = dp[i + 1].max(mx + ci);
        }
        // eprintln!("{dp:?}");
    }
    let ans = dp.iter().copied().max().unwrap();
    println!("{ans}");
}