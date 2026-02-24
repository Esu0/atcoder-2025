use proconio::{input, marker};

fn main() {
    input! {
        n: usize,
        mut s: [marker::Bytes; n],
    }

    let mut num = vec![Option::<u8>::None; n * 2];
    for i in 0..n {
        for j in 0..n {
            if s[i][j] != b'?' {
                if let Some(x) = num[i + j] && x != s[i][j] {
                    println!("-1");
                    return;
                } else {
                    num[i + j] = Some(s[i][j]);
                }
            }
        }
    }
    for i in 0..n {
        for j in 0..n {
            s[i][j] = num[i + j].unwrap_or(b'0');
        }
    }
    for si in &s {
        println!("{}", std::str::from_utf8(si).unwrap());
    }
}