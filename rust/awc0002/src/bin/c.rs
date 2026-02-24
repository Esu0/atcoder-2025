fn main() {
    proconio::input! {
        n: usize,
        m: u32,
        ab: [(u32, u32); n],
    }

    let mut ans = 0;
    for &(ai, bi) in &ab {
        ans = ans.max(m.saturating_sub(ai).div_ceil(bi));
    }
    println!("{ans}");
}