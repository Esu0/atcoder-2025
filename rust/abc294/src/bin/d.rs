#[proconio::fastout]
fn main() {
    proconio::input! {
        n: usize,
        q: usize,
    }

    let mut s = Vec::with_capacity(n);
    let mut ans = 0;
    for _ in 0..q {
        proconio::input! { t: u8 }
        if t == 1 {
            s.push(false);
        } else if t == 2 {
            proconio::input! { x: usize }
            s[x - 1] = true;
            while ans < s.len() && s[ans] { ans += 1 }
        } else {
            println!("{}", ans + 1);
        }
    }
}
