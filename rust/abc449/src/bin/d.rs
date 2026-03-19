#[allow(unused_imports)]
use proconio::{input, marker};

const L: i64 = -1000001;
fn count(y: i64, r: i64) -> i64 {
    let mut count = 0;
    let i = y.abs();
    let a = r.min(-i - 1) - L;
    count += (a + 1) / 2;
    if r >= -i && i % 2 == 0 {
        let a = i.min(r) + i + 1;
        count += a;
    }
    if r > i {
        let a = r - i;
        if i % 2 == 0 {
            count += a / 2;
        } else {
            count += (a + 1) / 2;
        }
    }
    count
}
fn main() {
    input! {
        l: i64,
        r: i64,
        d: i64,
        u: i64,
    }

    let mut ans = 0;
    for i in d..=u {
        ans += count(i, r) - count(i, l - 1);
    }
    println!("{ans}");
}
