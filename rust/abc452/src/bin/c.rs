#[allow(unused_imports)]
use proconio::{input, marker};

#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        ab: [(usize, usize); n],
        m: usize,
        s: [marker::Bytes; m],
    }
    let mut sets = vec![[false; 26]; n];
    for si in &s {
        for (i, &(ai, bi)) in ab.iter().enumerate() {
            if si.len() == ai {
                sets[i][(si[bi - 1] - b'a') as usize] = true;
            }
        }
    }
    for si in &s {
        if si.len() != n {
            println!("No");
            continue;
        }
        if si.iter().zip(&sets).all(|(&si, set)| set[(si - b'a') as usize]) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
