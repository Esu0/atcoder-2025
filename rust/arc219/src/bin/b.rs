#[allow(unused_imports)]
use proconio::{input, marker};

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            p: [usize; n],
        }

        let mut k = 0;
        while k < n && p[k] == k + 1 {
            k += 1;
        }
        if k == 0 {
            println!("0");
        } else if k < n {
            assert!(n > 2);
            assert!(k <= n - 2);
            let ans = (n - k + (n - 1)) * k / 2;
            println!("{}", ans % 998244353);
        } else {
            let ans = n * (n - 1) / 2 + 1;
            println!("{}", ans % 998244353);
        }
    }
}
