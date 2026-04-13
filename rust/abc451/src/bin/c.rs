use std::{cmp::Reverse, collections::BinaryHeap};

#[allow(unused_imports)]
use proconio::{input, marker};

#[proconio::fastout]
fn main() {
    input! {
        q: usize,
    }
    let mut queue = BinaryHeap::new();

    for _ in 0..q {
        input! {
            t: u8,
            h: u32,
        }
        if t == 1 {
            queue.push(Reverse(h));
        } else {
            while queue.peek().is_some_and(|&Reverse(x)| x <= h) {
                queue.pop();
            }
        }
        println!("{}", queue.len());
    }
}
