use proconio::input;
#[proconio::fastout]
fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }
    let mut cnt = vec![0u64; n];
    for &(ai, bi) in &ab {
        cnt[ai - 1] += 1;
        cnt[bi - 1] += 1;
    }
    for &c in &cnt {
        let k = n as u64 - c - 1;
        if k >= 3 {
            print!("{} ", k * (k - 1) * (k - 2) / 6);
        } else {
            print!("0 ");
        }
    }
    println!();
}