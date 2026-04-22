use std::collections::VecDeque;
use ac_library::{convolution::convolution, ModInt998244353 as MInt};

#[allow(unused_imports)]
use proconio::{input, marker};

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }

    let sum = a.iter().sum::<u32>();
    let mut queue = a.iter().map(|&ai| vec![MInt::raw(1), MInt::raw(ai)]).collect::<VecDeque<_>>();
    while queue.len() > 1 {
        let f = queue.pop_front().unwrap();
        let g = queue.pop_front().unwrap();
        let h = convolution(&f, &g);
        queue.push_back(h);
    }
    let f = queue.pop_front().unwrap();
    let mut ans = MInt::raw(1);
    let mut sck = MInt::raw(1);
    for k in 1..=n {
        sck = sck * MInt::raw(sum - (k - 1) as u32).inv() * MInt::raw(k as u32);
        ans += sck * f[k];
    }
    println!("{}", ans.val());
}
