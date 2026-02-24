use ac_library::MfGraph;

fn main() {
    proconio::input! {
        n: usize,
        a: usize,
        b: usize,
        mut s: [proconio::marker::Bytes; n],
    }

    let mut mf = MfGraph::new(n * n * 2 + 2);
    let mut col = vec![false; n * n];
    let mut vis = vec![false; n * n];
    let mut stack = vec![];
    for i in 0..n * n {
        if vis[i] { continue }
        vis[i] = true;
        stack.push(i);
        while let Some(u) = stack.pop() {
            let i = u / n;
            let j = u % n;
            for k in 0..8 {
                let swap = (k >> 2) != 0;
                let nega = ((k >> 1) & 1) != 0;
                let negb = k & 1 != 0;
                let (a, b) = if swap { (b, a) } else { (a, b) };
                let na = if nega { i.wrapping_sub(a) } else { i + a };
                let nb = if negb { j.wrapping_sub(b) } else { j + b };
                if (0..n).contains(&na) && (0..n).contains(&nb) {
                    let v = na * n + nb;
                    if !vis[v] {
                        stack.push(v);
                        vis[v] = true;
                        col[v] = !col[u];
                    } else {
                        // eprintln!("{} -> {}", u, v);
                        assert_ne!(col[u], col[v]);
                    }
                }
            }
        }
    }
    let mx = 1000000u64;
    let off = n * n;
    for i in 0..n {
        for j in 0..n {
            if s[i][j] == b'.' {
                for k in 0..8 {
                    let swap = (k >> 2) != 0;
                    let nega = ((k >> 1) & 1) != 0;
                    let negb = k & 1 != 0;
                    let (a, b) = if swap { (b, a) } else { (a, b) };
                    let na = if nega { i.wrapping_sub(a) } else { i + a };
                    let nb = if negb { j.wrapping_sub(b) } else { j + b };
                    if (0..n).contains(&na) && (0..n).contains(&nb) && s[na][nb] == b'.' {
                        let (mut u, mut v) = (i * n + j, na * n + nb);
                        assert_ne!(col[u], col[v]);
                        if col[u] {
                            (u, v) = (v, u);
                        }
                        mf.add_edge(u, v, mx);
                    }
                }
            }
            if col[i * n + j] {
                mf.add_edge(i * n + j, i * n + j + off, 1);
                mf.add_edge(i * n + j + off, off * 2 + 1, mx);
            } else {
                mf.add_edge(i * n + j + off, i * n + j, 1);
                mf.add_edge(off * 2, i * n + j + off, mx);
            }
        }
    }
    mf.flow(off * 2, off * 2 + 1);
    // eprintln!("flow: {flow}");
    let mincut = mf.min_cut(off * 2);
    // eprintln!("mincut: {mincut:?}");
    // let mut k = 0;
    for i in 0..n {
        for j in 0..n {
            // eprintln!("({i}, {j}): {}, {}", mincut[i*n+j], mincut[i*n+j+off]);
            if s[i][j] == b'#' {
                assert_eq!(mincut[i * n + j], mincut[i * n + j + off]);
                continue;
            }
            if mincut[i * n + j] == mincut[i * n + j + off] {
                s[i][j] = b'o';
                // k += 1;
            }
        }
        println!("{}", std::str::from_utf8(&s[i]).unwrap());
    }
    // eprintln!("count: {k}");
}