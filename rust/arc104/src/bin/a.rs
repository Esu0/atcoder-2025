#[allow(unused_imports)]
use proconio::{input, marker};

fn main() {
    input! {
        a: i32,
        b: i32,
    }
    println!("{} {}", (a + b) / 2, (a - b) / 2);
}
