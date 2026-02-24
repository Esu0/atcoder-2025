fn main() {
    proconio::input! {
        n: usize,
        t: u32,
        a: [u32; n],
    }

    let mut ans = 0;
    let mut open = 0;
    for &ai in &a {
        if ai > open {
            ans += ai - open;
            open = ai + 100;
        }
    }
    ans += t.saturating_sub(open);
    println!("{ans}");
}