#[allow(unused_imports)]
use proconio::{input, marker};

fn main() {
    input! {
        s: marker::Bytes,
    }
    if s.len() % 5 == 0 {
        println!("Yes");
    } else {
        println!("No");
    }
}
