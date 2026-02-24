use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            d: usize,
            a: [u32; n],
            b: [u32; n],
        }
        let mut que = VecDeque::new();
        for i in 0..n {
            for _ in 0..a[i] {
                que.push_back(i);
            }
            for _ in 0..b[i] {
                que.pop_front();
            }
            if i >= d {
                while let Some(a) = que.front() && *a <= i - d {
                    que.pop_front();
                }
            }
        }
        println!("{}", que.len());
    }
}
