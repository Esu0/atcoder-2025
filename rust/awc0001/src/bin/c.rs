fn main() {
    proconio::input! {
        n: usize,
        k: usize,
        mut d: [u64; n],
    }
    d.sort_unstable();
    println!("{}", d[..n-k].iter().sum::<u64>());
}