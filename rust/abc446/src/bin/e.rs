use proconio::input;

fn main() {
    input! {
        m: usize,
        a: usize,
        b: usize,
    }
    let mut g = vec![vec![]; m * m];
    let mut stack = Vec::with_capacity(m * m);
    let mut vis = vec![false; m * m];
    for p in 0..m {
        for pp in 0..m {
            let npp = p;
            let np = (p * a + pp * b) % m;
            g[np * m + npp].push(p * m + pp);
            if p == 0 || pp == 0 {
                stack.push(p * m + pp);
                vis[p * m + pp] = true;
            }
        }
    }
    while let Some(u) = stack.pop() {
        for &v in &g[u] {
            if vis[v] { continue }
            vis[v] = true;
            stack.push(v);
        }
    }

    let ans = vis.iter().filter(|&&f| !f).count();
    println!("{ans}");
}
