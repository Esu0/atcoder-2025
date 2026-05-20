#[allow(unused_imports)]
use proconio::{input, marker};

fn main() {
    input! {
        n: usize,
        a: [u32; n],
        x: usize,
    }
    println!("{}", a[x-1]);
}
