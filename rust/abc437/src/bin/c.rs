#[proconio::fastout]
fn main() {
    proconio::input! {
        t: usize,
    }
    for _ in 0..t {
        proconio::input! {
            n: usize,
            mut wp: [(u64, u64); n],
        }
        wp.sort_unstable_by_key(|&(wi, pi)| wi + pi);
        let mut sum = wp.iter().map(|&(_, pi)| pi).sum::<u64>();
        for (i, &(wi, pi)) in wp.iter().enumerate() {
            if sum < wi + pi {
                println!("{}", i);
                break;
            }
            sum -= wi + pi;
        }
    }
}