use std::collections::HashSet;

#[allow(unused_imports)]
use proconio::{input, marker};

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [u32; n],
    }
    let mut ia = a.iter().copied().enumerate().collect::<Vec<_>>();
    ia.sort_unstable_by_key(|&(_, ai)| ai);
    let mut set = HashSet::new();
    for _ in 0..q {
        input! { k: usize }
        set.clear();
        for _ in 0..k {
            input! { bi: usize }
            set.insert(bi - 1);
        }
        for &(i, ai) in &ia {
            if set.contains(&i) { continue }
            println!("{ai}");
            break;
        }
    }
}
