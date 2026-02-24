use proconio::input;

fn main() {
    input! {
        n: usize,
        k: u64,
        ab: [(u64, u64); n],
    }
    println!("{}", ab.iter().filter(|&&(ai, bi)| ai * bi >= k).count());
}
