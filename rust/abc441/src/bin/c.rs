use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        x: u64,
        mut a: [u64; n],
    }

    a.sort_unstable_by_key(|&ai| std::cmp::Reverse(ai));
    let mut s = 0;
    for (i, &ai) in a[n - k..].iter().enumerate() {
        s += ai;
        if s >= x {
            println!("{}", i + n - k + 1);
            return;
        }
    }
    println!("-1");
}