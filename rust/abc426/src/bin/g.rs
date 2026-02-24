#[allow(unused_imports)]
use proconio::{input, marker};

fn dfs(
    lrci: &mut [(usize, usize, usize, usize)],
    ans: &mut [u64],
    wv: &[(usize, u64)],
) {
    if lrci.is_empty() { return }
    if wv.len() == 1 {
        let (w, v) = wv[0];
        for &(li, ri, ci, i) in &*lrci {
            assert_eq!((li, ri), (0, 1));
            ans[i] = if w <= ci { v } else { 0 };
        }
        return;
    }
    let m = wv.len() / 2;
    let mut i = 0;
    for k in 0..lrci.len() {
        let (lk, rk, ck, ik) = lrci[k];
        if rk <= m {
            lrci[k] = lrci[i];
            lrci[i] = (lk, rk, ck, ik);
            i += 1;
        }
    }
    let mut j = i;
    for k in i..lrci.len() {
        let (lk, rk, ck, ik) = lrci[k];
        if m <= lk {
            lrci[k] = lrci[j];
            lrci[j] = (lk - m, rk - m, ck, ik);
            j += 1;
        }
    }
    let (lwv, rwv) = wv.split_at(m);
    dfs(&mut lrci[0..i], ans, lwv);
    dfs(&mut lrci[i..j], ans, rwv);

    let lrci = &lrci[j..];
    // eprintln!("{lrci:?}");
    // eprintln!("{lwv:?}");
    // eprintln!("{rwv:?}");
    let mut dpl = vec![[0; 501]; lwv.len() + 1];
    let mut dpr = vec![[0; 501]; rwv.len() + 1];
    for i in 0..lwv.len() {
        let (wi, vi) = lwv[lwv.len() - 1 - i];
        for j in 0..=500 {
            dpl[i + 1][j] = dpl[i][j];
        }
        for j in 0..=500 - wi {
            dpl[i + 1][j + wi] = dpl[i + 1][j + wi].max(dpl[i][j] + vi);
        }
        // eprintln!("{}: {:?}", i + 1, &dpl[i + 1][..10]);
    }
    for i in 0..rwv.len() {
        let (wi, vi) = rwv[i];
        for j in 0..=500 {
            dpr[i + 1][j] = dpr[i][j];
        }
        for j in 0..=500 - wi {
            dpr[i + 1][j + wi] = dpr[i + 1][j + wi].max(dpr[i][j] + vi);
        }
    }

    for &(li, ri, ci, i) in lrci {
        let dpl = &dpl[m - li];
        let dpr = &dpr[ri - m];
        let mut mx = 0;
        let mut a = 0;
        for j in 0..=ci {
            mx = mx.max(dpl[j]);
            a = a.max(mx + dpr[ci - j]);
        }
        ans[i] = a;
    }
}

fn main() {
    input! {
        n: usize,
        wv: [(usize, u64); n],
        q: usize,
        lrc: [(usize, usize, usize); q],
    }
    let mut lrci = lrc.iter().enumerate().map(|(i, &(li, ri, ci))| (li - 1, ri, ci, i)).collect::<Vec<_>>();
    let mut ans = vec![0; q];
    dfs(&mut lrci, &mut ans, &wv);
    for &ai in &ans {
        println!("{ai}");
    }
}
