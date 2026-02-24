fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        k: u64,
        a: [u64; n],
        b: [usize; m],
    }

    let mut sum = 0;
    let mut cnt = 0;
    for &bi in &b {
        if a[bi-1] < k {
            sum += a[bi - 1];
            cnt += 1;
        }
    }
    println!("{cnt} {sum}");
}