#[allow(unused_imports)]
use proconio::{input, marker};

fn main() {
    input! {
        n: usize,
        m: usize,
        c: [u32; m],
        ab: [(usize, u32); n],
    }

    let mut sum = vec![0; m];
    for &(ai, bi) in &ab {
        sum[ai - 1] += bi;
    }
    let mut ans = 0;
    for (&ci, &si) in c.iter().zip(&sum) {
        ans += ci.min(si);
    }
    println!("{ans}");
}
