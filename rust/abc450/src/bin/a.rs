#[allow(unused_imports)]
use proconio::{input, marker};

fn main() {
    input! {
        n: u32,
    }
    print!("{n}");
    for i in (1..n).rev() {
        print!(",{i}");
    }
    println!();
}
