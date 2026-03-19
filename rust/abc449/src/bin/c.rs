#[allow(unused_imports)]
use proconio::{input, marker};

fn main() {
    input! {
        n: usize,
        l: usize,
        r: usize,
        s: marker::Bytes,
    }

    let mut ans = 0usize;
    let mut set = [0; 26];
    for &si in &s[l..=r] {
        set[(si - b'a') as usize] += 1;
    }
    let mut j = l;
    let mut k = r;
    for i in 0..n {
        let si = (s[i] - b'a') as usize;
        ans += set[si];
        if k + 1 < n {
            k += 1;
            set[(s[k] - b'a') as usize] += 1;
        }
        if j < n {
            set[(s[j] - b'a') as usize] -= 1;
            j += 1;
        }
    }
    println!("{ans}");
}
