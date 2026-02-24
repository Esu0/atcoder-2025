use proconio::input;

fn main() {
    input! {
        n: usize,
        lr: [(char, char); n],
    }
    let mut ans = 0;
    for i in 0..n - 1 {
        if lr[i].1 == lr[i + 1].0 {
            ans += 1;
        }
    }
    println!("{ans}");
}
