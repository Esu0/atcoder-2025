#[proconio::fastout]
fn main() {
    proconio::input! { t: usize }

    let mut stack = vec![];
    for _ in 0..t {
        proconio::input! {
            n: usize,
            m: usize,
            abx: [(usize, usize, usize); m],
        }

        if n % 2 == 1 {
            println!("-1");
            continue;
        }

        let mut g = vec![vec![]; n];
        for &(ai, bi, xi) in &abx {
            g[ai - 1].push((bi - 1, xi));
            g[bi - 1].push((ai - 1, xi));
        }

        stack.clear();
        let mut val = vec![usize::MAX; n];
        stack.push(0);
        val[0] = 0;
        while let Some(u) = stack.pop() {
            let d = val[u];
            for &(v, w) in &g[u] {
                if val[v] != usize::MAX {
                    assert_eq!(val[v], d ^ w);
                } else {
                    val[v] = d ^ w;
                    stack.push(v);
                }
            }
        }

        let mut ans = 0;
        for i in 1..=n {
            ans ^= i;
        }
        val.iter().for_each(|&v| ans ^= v);
        assert!(ans <= n);
        println!("{ans}");
    }

    // for n in 2..=100 {
    //     if n % 2 == 1 { continue }
    //     for r in 0..=n {
    //         let mut arr = vec![0; n];
    //         for i in 0..n {
    //             arr[i] = if i < r { i } else { i + 1 };
    //         }
    //         for i in 0..=n {
    //             let mut new = arr.clone();
    //             new.iter_mut().for_each(|ai| *ai ^= i);
    //             new.sort_unstable();
    //             if new.iter().all(|&ai| ai <= n) {
    //                 assert_eq!(new, arr);
    //             }
    //         }
    //     }
    // }

}