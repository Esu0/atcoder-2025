#[allow(unused_imports)]
use proconio::{input, marker};

fn main() {
    input! {
        n: usize,
        mut k: usize,
        a: [[u32]; n],
        c: [usize; n],
    }

    for i in 0..n {
        if a[i].len() * c[i] >= k {
            println!("{}", a[i][(k - 1) % a[i].len()]);
            return;
        }
        k -= a[i].len() * c[i];
    }
}
