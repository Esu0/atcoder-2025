use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        lrx: [(usize, usize, usize); m],
    }

    let mut g = vec![vec![]; n + 1];
    for i in 0..n {
        g[i].push((i + 1, 1));
        g[i + 1].push((i, 0));
    }

    for &(li, ri, xi) in &lrx {
        g[li - 1].push((ri, ri - li + 1 - xi));
    }

    let mut queue = BinaryHeap::from([(Reverse(0), 0)]);
    let mut dist = vec![usize::MAX; n + 1];
    while let Some((Reverse(d), u)) = queue.pop() {
        if dist[u] != usize::MAX { continue }
        dist[u] = d;
        for &(v, w) in &g[u] {
            queue.push((Reverse(d + w), v));
        }
    }

    // eprintln!("{dist:?}");
    for i in 0..n {
        let c = if dist[i] == dist[i + 1] { '1' } else {
            assert_eq!(dist[i] + 1, dist[i + 1]);
            '0'
        };
        print!("{} ", c);
    }
    println!();
}