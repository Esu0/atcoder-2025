use proconio::input;
use ac_library::{ModInt998244353 as MInt, convolution::convolution};

fn main() {
    input! {
        n: usize,
    }

    let mut f = vec![MInt::raw(0); 2 * n + 1];
    let mut g = vec![MInt::raw(0); 2 * n + 1];
    let mut h = vec![MInt::raw(0); 2 * n + 1];
    f[0] = MInt::raw(1);
    h[0] = MInt::raw(1);
    g[0] = MInt::raw(1);
    let mut fact = MInt::raw(1);
    for i in 1..n {
        fact *= MInt::raw((i + 1) as u32);
        g[i] = fact;
        f[i] = fact - h[i - 1];
        let mut k = i + 1;
        let mut l = 1;
        let mut ofs = 0;
        while k % 2 == 1 {
            // dbg!(k, l, ofs);
            if k == 1 {
                let tmp = convolution(&f[ofs..ofs + l], &g[ofs..ofs + l]);
                for (hi, &v) in h[i..].iter_mut().zip(&tmp) {
                    *hi += v;
                }
            } else {
                let tmp = convolution(&f[ofs..ofs+l], &g[i - ofs..i - ofs + l]);
                for (hi, &v) in h[i..].iter_mut().zip(&tmp) {
                    *hi += v;
                }
                let tmp = convolution(&f[i - ofs..i - ofs + l], &g[ofs..ofs + l]);
                for (hi, &v) in h[i..].iter_mut().zip(&tmp) {
                    *hi += v;
                }
            }
            ofs += l;
            l *= 2;
            k >>= 1;
        }
        if k != 0 {
            // dbg!(k, ofs, l);
            let tmp = convolution(&f[ofs..ofs+l], &g[i - ofs..i - ofs + l]);
            for (hi, &v) in h[i..].iter_mut().zip(&tmp) {
                *hi += v;
            }
            let tmp = convolution(&f[i - ofs..i - ofs + l], &g[ofs..ofs + l]);
            for (hi, &v) in h[i..].iter_mut().zip(&tmp) {
                *hi += v;
            }
        }
    }
    // eprintln!("{f:?}");
    // eprintln!("{h:?}");
    println!("{}", f[n - 1].val());
}