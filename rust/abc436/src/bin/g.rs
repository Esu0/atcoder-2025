use std::collections::VecDeque;

use proconio::input;
use ac_library::{convolution::convolution, ModInt998244353 as MInt};

fn main() {
    input! {
        n: usize,
        m: u64,
        a: [usize; n],
    }

    let mut queue = VecDeque::from([vec![MInt::raw(1), MInt::new(-1)]]);
    for &ai in &a {
        let mut v = vec![MInt::raw(0); ai + 1];
        v[ai] = MInt::new(-1);
        v[0] = MInt::new(1);
        queue.push_back(v);
    }
    while queue.len() >= 2 {
        let f = queue.pop_front().unwrap();
        let g = queue.pop_front().unwrap();
        let h = convolution(&f, &g);
        queue.push_back(h);
    }

    let mut f = queue.pop_back().unwrap();
    let mut g = vec![MInt::raw(1)];
    let mut m = m;
    while m > 0 {
        let mut nf = f.clone();
        nf.iter_mut().skip(1).step_by(2).for_each(|c| *c = -*c);
        {
            let res = convolution(&f, &nf);
            f.clear();
            f.extend(res.iter().copied().step_by(2));
        }
        let h = convolution(&g, &nf);
        g.clear();
        if m & 1 == 0 {
            g.extend(h.iter().copied().step_by(2));
        } else {
            g.extend(h.iter().copied().skip(1).step_by(2));
        }
        // eprintln!("{g:?}");
        m >>= 1;
    }
    println!("{}", g[0] / f[0]);
}
