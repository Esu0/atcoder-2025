#[allow(unused_imports)]
use proconio::{input, marker};

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize; n],
        q: usize,
    }

    let mut count = (1..=m).map(|i| (0usize, i)).collect::<Vec<_>>();
    for &ai in &a {
        count[ai - 1].0 += 1;
    }
    count.retain(|x| x.0 != 0);
    count.sort_by_key(|x| x.0);
    let mut cumsum = vec![0; count.len() + 1];
    for i in 0..count.len() {
        cumsum[i + 1] = cumsum[i] + count[i].0;
    }

    for _ in 0..q {
        input! { x: usize }
        if x <= n {
            println!("{}", a[x - 1]);
            continue;
        }
        let x = x - n;
        let mut ok = 0;
        let mut ng = 2_000_000_000_000_000_000usize.div_ceil(count.len());
        while ng - ok > 1 {
            let mid = (ng + ok) / 2;
            let mut c = 0;
            
        }
    }
}
