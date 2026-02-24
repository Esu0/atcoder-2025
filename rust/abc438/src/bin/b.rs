use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: proconio::marker::Bytes,
        t: proconio::marker::Bytes,
    }

    let mut ans = u32::MAX;
    for i in 0..=n - m {
        let mut cnt = 0;
        for (&si, &ti) in s[i..i + m].iter().zip(&t) {
            cnt += (si - b'0' + 10 - (ti - b'0')) as u32 % 10;
        }
        ans = ans.min(cnt);
    }
    println!("{ans}");
}
