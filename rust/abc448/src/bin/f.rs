use std::cmp::Reverse;

#[allow(unused_imports)]
use proconio::{input, marker};

const B: u32 = 80000;

fn main() {
    input! {
        n: usize,
        mut xy: [(u32, u32); n],
    }

    let mut blocks = vec![vec![]; 20_000_001u32.div_ceil(B) as usize];
    for (i, &(xi, yi)) in xy.iter().enumerate() {
        blocks[(xi / B) as usize].push((i, (xi, yi)));
    }
    blocks.iter_mut().step_by(2).for_each(|block| block.sort_unstable_by_key(|&(_, (_, yi))| yi));
    blocks.iter_mut().skip(1).step_by(2).for_each(|block| block.sort_unstable_by_key(|&(_, (_, yi))| Reverse(yi)));
    let mut ixy = Vec::with_capacity(n);
    for block in &blocks {
        ixy.extend_from_slice(block);
    }

    let mut s = usize::MAX;
    let mut d = 0;
    let mut prev = ixy.last().unwrap().1;

    for (j, &(i, (xi, yi))) in ixy.iter().enumerate() {
        if i == 0 { s = j }
        d += prev.0.abs_diff(xi) as u64 + prev.1.abs_diff(yi) as u64;
        prev = (xi, yi);
    }
    assert!(d <= 10_000_000_000);
    ixy.rotate_left(s);
    for &(i, _) in &ixy {
        print!("{} ", i + 1);
    }
    println!();
}
