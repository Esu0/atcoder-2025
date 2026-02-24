use proconio::input;

fn main() {
    input! {
        t: usize,
    }
    'next: for _ in 0..t {
        input! {
            n: usize,
            xyz: [(usize, usize, usize); n],
        }
        let mut yz = vec![vec![]; n];
        for &(xi, yi, zi) in &xyz {
            yz[xi - 1].push((yi, zi));
        }
        let mut acc = (0, 0);
        let mut pref = vec![(0, 0); n];
        for x in 0..n {
            pref[x] = acc;
            for &(yi, zi) in &yz[x] {
                let (ymax, zmax) = acc;
                acc = (ymax.max(yi), zmax.max(zi));
            }
        }
        acc = (usize::MAX, usize::MAX);
        let mut count = 0u32;
        for x in (0..n).rev() {
            for &(yi, zi) in &yz[x] {
                let (ymin, zmin) = acc;
                acc = (ymin.min(yi), zmin.min(zi));
                count += 1;
            }
            if acc.0 == usize::MAX { continue }
            if acc.0 > pref[x].0 && acc.1 > pref[x].1 {
                println!("{count}");
                continue 'next;
            }
        }
        unreachable!();
    }
}
