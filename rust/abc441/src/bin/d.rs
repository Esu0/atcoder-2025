use proconio::input;

fn dfs(u: usize, cnt: u8, cost: u32, s: u32, t: u32, g: &[Vec<(usize, u32)>], ans: &mut [bool]) {
    if cnt == 0 {
        if (s..=t).contains(&cost) {
            ans[u] = true;
        }
        return;
    }

    for &(v, c) in &g[u] {
        dfs(v, cnt - 1, cost + c, s, t, g, ans);
    }
}


fn main() {
    input! {
        n: usize,
        m: usize,
        l: u8,
        s: u32,
        t: u32,
        uvc: [(usize, usize, u32); m],
    }

    let mut g = vec![vec![]; n];
    for &(ui, vi, ci) in &uvc {
        g[ui - 1].push((vi - 1, ci));
    }
    let mut ans = vec![false; n];
    dfs(0, l, 0, s, t, &g, &mut ans);
    for (i, &f) in ans.iter().enumerate() {
        if f {
            print!("{} ", i + 1);
        }
    }
    println!();

}