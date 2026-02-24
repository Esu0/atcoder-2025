#[allow(unused_imports)]
use proconio::{input, marker};

fn main() {
    input! {
        n: usize,
        w: [u32; n],
        q: usize,
    }

    let mut ranges = vec![(0, n, 0, false)];
    for _ in 0..q {
        input! { t: u8 }
        if t == 1 {
            input! { v: usize }
            let v = v - 1;
            if v == 0 { continue }
            let d = loop {
                let (l, r, x, u) = ranges.pop().unwrap();
                if (l..r).contains(&v) {
                    ranges.push((v, r, x, u));
                    break if u { x - w[v] } else { x };
                }
            };
            ranges.push((0, v, d, false));
        } else if t == 2 {
            input! { v: usize }
            let v = v - 1;
            if v == 0 { continue }
            let u = loop {
                let (l, r, x, u) = ranges.pop().unwrap();
                if (l..r).contains(&v) {
                    ranges.push((v, r, x, u));
                    break if u { x } else { x + w[v] };
                }
            };
            ranges.push((0, v, u, true));
        } else {
            input! { x: u32 }
            let v = ranges.partition_point(|&(_, r, y, u)| {
                let (u, d) = if u {
                    (y, y - w[r - 1])
                } else {
                    (y + w[r - 1], y)
                };
                (d..u).contains(&x)
            });
            // dbg!(v);
            let ans = if v == 0 {
                0
            } else {
                let (l, r, y, u) = ranges[v - 1];
                let k = w[l..r].partition_point(|&wi| {
                    let (u, d) = if u {
                        (y, y - wi)
                    } else {
                        (y + wi, y)
                    };
                    !(d..u).contains(&x)
                });
                n - r + (r - (l + k))
            };
            println!("{ans}");
        }
        // eprintln!("{ranges:?}");
    }
}
