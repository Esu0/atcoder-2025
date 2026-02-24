use std::{cmp::Reverse, collections::BinaryHeap};

fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        xyc: [(usize, usize, u64); m],
    }
    let mut g = vec![vec![]; n];
    for &(xi, yi, ci) in &xyc[1..] {
        g[xi - 1].push((yi - 1, ci));
    }
    let (x0, y0, c0) = xyc[0];
    let mut dist = vec![u64::MAX; n];
    let mut queue = BinaryHeap::from([(Reverse(c0), y0 - 1)]);
    while let Some((Reverse(d), u)) = queue.pop() {
        if dist[u] != u64::MAX {
            continue;
        }
        dist[u] = d;
        for &(v, w) in &g[u] {
            queue.push((Reverse(d + w), v));
        }
    }

    println!("{}", dist[x0 - 1] as i64);
}
