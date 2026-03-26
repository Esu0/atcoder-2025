#[allow(unused_imports)]
use proconio::{input, marker};

fn main() {
    input! {
        n: usize,
        s: marker::Bytes,
    }
    let mut ans = 0u32;
    for l in 0..n {
        let mut cnt = [0u32; 26];
        for r in l..n {
            cnt[(s[r] - b'A') as usize] += 1;
            if cnt[0] == cnt[(b'T' - b'A') as usize] && cnt[2] == cnt[(b'G' - b'A') as usize] {
                ans += 1;
            }
        }
    }
    println!("{ans}");
}
