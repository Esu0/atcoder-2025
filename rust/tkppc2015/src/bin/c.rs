use proconio::input;

fn main() {
    input! {
        n: usize,
        m: i32,
        s: u32,
        mut tk: [(u32, i32); n],
    }
    tk.sort_unstable_by_key(|&(ti, _)| ti);
    let mut a = 0;
    let mut ans = 0;
    let mut prev = 0;
    for &(ti, ki) in &tk {
        if a >= m {
            ans += ti - prev;
        }
        a += ki;
        prev = ti;
    }
    if a >= m {
        ans += s - prev;
    }
    println!("{ans}");
}
