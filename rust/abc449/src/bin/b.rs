#[allow(unused_imports)]
use proconio::{input, marker};

fn main() {
    input! {
        mut h: usize,
        mut w: usize,
        q: usize,
    }
    for _ in 0..q {
        input! {
            t: u8,
            x: usize,
        }
        let ans;
        if t == 1 {
            ans = w * x;
            h -= x;
        } else {
            ans = h * x;
            w -= x;
        }
        println!("{ans}");
    }
}
