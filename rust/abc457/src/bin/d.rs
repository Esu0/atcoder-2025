#[allow(unused_imports)]
use proconio::{input, marker};

fn main() {
    input! {
        n: usize,
        k: u64,
        a: [u64; n],
    }

    let mut ok = 1;
    let mut ng = u64::MAX;
    while ng - ok > 1 {
        let m = (ng - ok) / 2 + ok;
        let mut cnt = 0;
        for (i, &ai) in a.iter().enumerate() {
            let i = (i + 1) as u64;
            cnt += m.saturating_sub(ai).div_ceil(i);
            if cnt > k {
                break;
            }
        }
        if cnt <= k {
            ok = m;
        } else {
            ng = m;
        }
    }
    println!("{ok}");
}
