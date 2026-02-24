use proconio::input;
fn main() {
    input! {
        n: usize,
        m: usize,
        pv: [(usize, i64); n],
    }

    let mut dp = vec![vec![i64::MIN; m + 1]; n + 1];
    dp[0][0] = 0;
    for (i, &(pi, vi)) in pv.iter().enumerate() {
        for j in 0..=m {
            dp[i + 1][j] = dp[i][j];
        }
        for j in 0..=m {
            if j + pi <= m {
                dp[i + 1][j + pi] = dp[i + 1][j + pi].max(dp[i][j] + vi);
            }
        }
        for j in 0..m {
            dp[i][j + 1] = dp[i][j + 1].max(dp[i][j]);
        }
    }

    // eprintln!("{dp:?}");
    let mx = dp[n].iter().copied().max().unwrap();
    // eprintln!("max:{mx}");

    let mut dpi = vec![i64::MIN; m + 1];
    let mut dpin = dpi.clone();
    dpi[0] = 0;
    let mut ans = vec![0; n];
    for (i, &(pi, vi)) in pv.iter().enumerate().rev() {
        let mut ok0 = false;
        let mut ok1 = false;
        // eprintln!("{dpi:?}");
        for k in 0..=m {
            if k + pi <= m {
                let val = dp[i][m - k - pi].saturating_add(dpi[k]) + vi;
                assert!(val <= mx);
                if val == mx {
                    ok0 = true;
                }
            }

            let val = dp[i][m - k].saturating_add(dpi[k]);
            assert!(val <= mx);
            if val == mx {
                ok1 = true;
            }
        }
        // eprintln!("{i}");
        assert!(ok0 || ok1);
        if !ok0 {
            ans[i] = b'C';
        } else if ok0 && ok1 {
            ans[i] = b'B';
        } else {
            assert!(ok0 && !ok1);
            ans[i] = b'A';
        }

        dpin.clone_from_slice(&dpi);
        for j in 0..=m - pi {
            dpin[j + pi] = dpin[j + pi].max(dpi[j] + vi);
        }
        dpi.clone_from(&dpin);
    }
    
    println!("{}", std::str::from_utf8(&ans).unwrap());
}