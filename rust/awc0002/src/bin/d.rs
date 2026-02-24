fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        mut c: [u32; n],
        mut r: [u32; m],
    }

    c.sort_unstable();
    r.sort_unstable();
    let mut ok = 0;
    let mut ng = n.min(m) + 1;
    while ng - ok > 1 {
        let mid = (ok + ng) / 2;
        if c.iter().zip(&r[m - mid..]).all(|(&ci, &ri)| ci <= ri) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    println!("{ok}");
}