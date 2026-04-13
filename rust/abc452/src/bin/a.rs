#[allow(unused_imports)]
use proconio::{input, marker};

fn main() {
    input! {
        m: u32,
        d: u32,
    }
    let ans = [(1, 7), (3, 3), (5, 5), (7, 7), (9, 9)];
    if ans.contains(&(m, d)) {
        println!("Yes");
    } else {
        println!("No");
    }
}
