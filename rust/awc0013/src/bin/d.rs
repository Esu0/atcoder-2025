#[allow(unused_imports)]
use proconio::{input, marker};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [[i32; m]; n],
    }

    let mut b = vec![vec![0; n]; m];
    for (i, ai) in a.iter().enumerate() {
        for (j, &aij) in ai.iter().enumerate() {
            b[j][i] = aij;
        }
    }

    let mut ans: u64 = 0;
    b.iter_mut().for_each(|bi| bi.sort_unstable());
    for bi in b.iter() {
        for i in 1..n {
            ans += i as u64 * (n - i) as u64 * (bi[i] - bi[i - 1]) as u64;
        }
    }
    println!("{ans}");
}
