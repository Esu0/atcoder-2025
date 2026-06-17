use proconio::input;
use ac_library::convolution::convolution;
use ac_library::ModInt998244353 as MInt;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        b: [usize; m],
    }
    let mut f = vec![MInt::raw(0); 500_001];
    let mut g = vec![MInt::raw(1); 500_001];
    let mut fact = MInt::raw(1);
    for i in 1..=500_000 {
        fact *= MInt::raw(i);
    }

    let mut fact_inv = fact.inv();
    for &bi in &b {
        f[bi] += 1;
    }
    for i in (1..=500_000).rev() {
        g[i as usize] = fact_inv;
        f[i as usize] *= fact_inv;
        fact_inv *= MInt::raw(i);
    }

    let h = convolution(&f, &g);
    let mut ca = vec![0; 500_001];
    for &ai in &a {
        ca[ai] += 1;
    }
    let mut ans = MInt::raw(0);
    fact = MInt::raw(1);
    for i in 1..=500_000 {
        fact *= MInt::raw(i as u32);
        ans += fact * h[i] * MInt::raw(ca[i]);
    }
    println!("{}", ans.val());

}