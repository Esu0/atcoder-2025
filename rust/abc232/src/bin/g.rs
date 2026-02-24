use std::{cmp::Reverse, collections::BinaryHeap};

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: u64,
        a: [u64; n],
        b: [u64; n],
    }

    let mut arr = a.iter().map(|&ai| (m - ai) % m).chain(b.iter().copied()).collect::<Vec<_>>();
    arr.sort_unstable();
    arr.dedup();
    let mut g = vec![vec![]; arr.len() + n];
    for i in 0..arr.len() - 1 {
        g[i].push((i + 1, arr[i + 1] - arr[i]));
    }
    g[arr.len() - 1].push((0, arr[0] + m - *arr.last().unwrap()));
    for (i, (&ai, &bi)) in a.iter().zip(&b).enumerate() {
        let k = arr.binary_search(&((m - ai) % m)).unwrap_or_else(|_| unreachable!());
        g[i + arr.len()].push((k, 0));
        let k = arr.binary_search(&bi).unwrap_or_else(|_| unreachable!());
        g[k].push((i + arr.len(), 0));
    }

    let mut queue = BinaryHeap::from([(Reverse(0), arr.len())]);
    let mut dist = vec![u64::MAX; g.len()];
    while let Some((Reverse(d), u)) = queue.pop() {
        if dist[u] != u64::MAX { continue }
        dist[u] = d;
        for &(v, w) in &g[u] {
            queue.push((Reverse(d + w), v));
        }
    }
    println!("{}", dist.last().unwrap());
}