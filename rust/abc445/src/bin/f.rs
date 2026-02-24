fn main() {
    proconio::input! {
        n: usize,
        mut k: u64,
        mut c: [[u64; n]; n],
    }
    let mut buf = vec![vec![0; n]; n];
    let mut ans = vec![vec![u64::MAX; n]; n];
    for i in 0..n {
        ans[i][i] = 0;
    }
    while k > 0 {
        if k & 1 != 0 {
            for i in 0..n {
                for j in 0..n {
                    let mut t = u64::MAX;
                    for k in 0..n {
                        t = t.min(c[i][k].saturating_add(ans[k][j]));
                    }
                    buf[i][j] = t;
                }
            }
            ans.iter_mut().zip(&buf).for_each(|(ansi, bufi)| ansi.clone_from_slice(bufi));
        }
        for i in 0..n {
            for j in 0..n {
                let mut t = u64::MAX;
                for k in 0..n {
                    t = t.min(c[i][k].saturating_add(c[k][j]));
                }
                buf[i][j] = t;
            }
        }
        c.iter_mut().zip(&buf).for_each(|(ci, bufi)| ci.clone_from_slice(bufi));
        k >>= 1;
    }
    for i in 0..n {
        println!("{}", ans[i][i]);
    }
}
