// use std::collections::VecDeque;

fn main() {
    proconio::input! {
        t: usize,
    }
    // let mut queue = VecDeque::new();
    for _ in 0..t {
        proconio::input! {
            n: usize,
            r: [usize; n],
        }

        let mut pos = vec![vec![]; n];
        for (i, &ri) in r.iter().enumerate() {
            pos[ri - 1].push(i);
        }

        let mut vis = vec![false; n];
        let mut ans = 0;
        for i in 0..n {
            for &p in &pos[i] {
                vis[p] = true;
            }
            for j in 0..pos[i].len() {
                let p = pos[i][j];
                if p > 0 && !vis[p - 1] {
                    ans += (r[p - 1] - 1) - (i + 1);
                    pos[i + 1].push(p - 1);
                    vis[p - 1] = true;
                }
                if p < n - 1 && !vis[p + 1] {
                    ans += (r[p + 1] - 1) - (i + 1);
                    pos[i + 1].push(p + 1);
                    vis[p + 1] = true;
                }
            }
        }
        println!("{ans}");
    }
}