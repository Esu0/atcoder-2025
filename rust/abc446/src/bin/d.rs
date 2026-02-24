use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut dp = HashMap::<usize, usize>::new();
    for &ai in &a {
        let v = dp.get(&(ai - 1)).copied().unwrap_or_default();
        let r = dp.entry(ai).or_insert(0);
        if *r < v + 1 {
            *r = v + 1;
        }
    }
    let ans = dp.iter().map(|(_, &v)| v).max().unwrap();
    println!("{ans}");
}
