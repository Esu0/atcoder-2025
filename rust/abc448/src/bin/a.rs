#[allow(unused_imports)]
use proconio::{input, marker};

fn main() {
    input! {
        n: usize,
        mut x: u32,
        a: [u32; n],
    }
    for &ai in &a {
        if x > ai {
            println!("1");
            x = ai;
        } else {
            println!("0");
        }
    }
}
