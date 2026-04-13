#[allow(unused_imports)]
use proconio::{input, marker};

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); n],
    }
    let mut ans = vec![0; m];
    for &(ai, bi) in &ab {
        ans[ai - 1] -= 1;
        ans[bi - 1] += 1;
    }
    ans.iter().for_each(|&x| println!("{x}"));
}
