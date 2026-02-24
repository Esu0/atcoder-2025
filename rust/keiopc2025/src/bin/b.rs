fn main() {
    proconio::input! {
        n: usize,
        k: usize,
        mut a: [u64; n],
    }
    for i in 0..n {
        a[i] *= (n - i) as u64;
    }
    let (_, _, z) = a.select_nth_unstable_by_key(k - 1, |&x| std::cmp::Reverse(x));
    println!("{}", z.iter().sum::<u64>());
}