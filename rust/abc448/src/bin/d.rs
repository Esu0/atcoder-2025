use std::collections::HashSet;

#[allow(unused_imports)]
use proconio::{input, marker};

fn dfs(g: &[Vec<usize>], a: &[u32], p: usize, u: usize, set: &mut HashSet<u32>, dp: &mut [bool], cond: bool) {
    if cond || set.contains(&a[u]) {
        dp[u] = true;
        for &v in &g[u] {
            if v == p { continue }
            dfs(g, a, u, v, set, dp, true);
        }
        return;
    }
    set.insert(a[u]);
    for &v in &g[u] {
        if v == p { continue }
        dfs(g, a, u, v, set, dp, false);
    }
    set.remove(&a[u]);
}

fn main() {
    input! {
        n: usize,
        a: [u32; n],
        uv: [(usize, usize); n - 1],
    }

    let mut g = vec![vec![]; n];
    for &(ui, vi) in &uv {
        g[ui - 1].push(vi - 1);
        g[vi - 1].push(ui - 1);
    }
    let mut set = HashSet::new();
    let mut dp = vec![false; n];
    dfs(&g, &a, usize::MAX, 0, &mut set, &mut dp, false);
    for &di in &dp {
        if di {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
