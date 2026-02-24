use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(usize, usize); m],
    }
    let mut g = vec![vec![]; n];
    for &(ui, vi) in &uv {
        g[ui - 1].push(vi - 1);
    }

    let mut ans = vec![usize::MAX; n];
    let mut queue = BTreeSet::from([0]);
    let mut vis = vec![false; n];
    let mut mn = 0;
    let mut mx = 0;
    while let Some(u) = queue.pop_first() {
        if !vis[u] {
            vis[u] = true;
            mx = mx.max(u + 1);
            while mn < n && vis[mn] {
                mn += 1;
            }
            for &v in &g[u] {
                if !vis[v] {
                    queue.insert(v);
                }
            }
        }
        // eprintln!("{u}: {mx} {mn} {queue:?}");
        if mx == mn && !queue.first().is_some_and(|&x| x < mx) {
            ans[mx - 1] = queue.len();
        }
    }

    for &ai in &ans {
        println!("{}", ai as isize);
    }
}
