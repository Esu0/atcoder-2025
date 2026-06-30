fn main() {
    proconio::input! {
        n: usize,
        k: usize,
        mut d: [u64; n],
    }
    if k != 0 {
        d.select_nth_unstable(n - k);
    }
    println!("{}", d[..n-k].iter().sum::<u64>());
}