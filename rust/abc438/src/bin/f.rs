use std::collections::HashSet;

use proconio::input;

fn dfs1(u: usize, g: &[Vec<usize>], sp: &mut [bool], vis: &mut [bool]) -> bool {
    vis[u] = true;
    if u == 1 {
        sp[u] = true;
        return true;
    }
    for &v in &g[u] {
        if vis[v] {
            continue;
        }
        if dfs1(v, g, sp, vis) {
            sp[u] = true;
            return true;
        }
    }
    false
}

fn dfs_get_set(
    u: usize,
    g: &[Vec<usize>],
    sp: &[bool],
    set: &mut [bool],
    cnt: &mut usize,
) {
    set[u] = true;
    *cnt += 1;
    for &v in &g[u] {
        if set[v] || sp[v] {
            continue;
        }
        dfs_get_set(v, g, sp, set, cnt);
    }
}

fn dfs2(
    u: usize,
    to: usize,
    g: &[Vec<usize>],
    sp: &mut [bool],
    set: &mut [bool],
    cnt: &mut usize,
) -> bool {
    assert!(set[u]);
    set[u] = false;
    *cnt -= 1;
    if u == to {
        set[u] = true;
        *cnt += 1;
        return true;
    }
    for &v in &g[u] {
        if !set[v] {
            continue;
        }
        if dfs2(v, to, g, sp, set, cnt) {
            sp[u] = true;
            for &w in &g[u] {
                if w == v || !set[w] {
                    continue;
                }
                dfs3(w, g, set, cnt);
            }
            // todo
            return true;
        }
    }
    false
}

fn dfs3(
    u: usize,
    g: &[Vec<usize>],
    nvis: &mut [bool],
    nvis_cnt: &mut usize,
) {
    assert!(nvis[u]);
    nvis[u] = false;
    *nvis_cnt -= 1;
    for &v in &g[u] {
        if nvis[v] {
            dfs3(v, &g, nvis, nvis_cnt);
        }
    }
}
fn main() {
    input! {
        n: usize,
        uv: [(usize, usize); n - 1],
    }

    let mut g = vec![vec![]; n];
    for &(ui, vi) in &uv {
        g[ui].push(vi);
        g[vi].push(ui);
    }
    let mut vis = vec![false; n];
    let mut gti = vec![0; n + 1];
    let mut sp = vec![false; n];
    let mut setu = vec![false; n];
    let mut cntu = 0;
    let mut setv = vec![false; n];
    let mut cntv = 0;
    assert!(dfs1(0, &g, &mut sp, &mut vis));
    dfs_get_set(0, &g, &sp, &mut setu, &mut cntu);
    dfs_get_set(1, &g, &sp, &mut setv, &mut cntv);
    let mut u = 0;
    let mut v = 1;
    let mut i = 2;
    let mut end = false;
    eprintln!("{setu:?}");
    eprintln!("{setv:?}");
    while i < n {
        if sp[i] {
            gti[i + 1] = cntu * cntv;
        } else if setu[i] {
            assert!(dfs2(u, i, &g, &mut sp, &mut setu, &mut cntu));
        } else if setv[i] {
            assert!(dfs2(v, i, &g, &mut sp, &mut setv, &mut cntv));
        } else {
            break;
        }
    }
    
    let mut uf = unionfind::Unionfind::new(vec![(); n]);
    for &(ui, vi) in &uv {
        if ui != 0 && vi != 0 {
            uf.unite(ui, vi);
        }
    }
    vis.fill(false);
    for i in 1..n {
        if vis[i] {
            continue;
        }
        vis[i] = true;
    }
}

pub mod unionfind {
    use std::ops::{Add, Sub};

    pub struct Unionfind<T> {
        uf: Vec<usize>,
        size: Vec<usize>,
        query: Vec<T>,
    }

    pub trait Query {
        fn query(&self, other: &Self) -> Self;
    }

    pub trait RevQuery: Query {
        /// other.query(output) == self
        fn rev_query(&self, other: &Self) -> Self;
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
    pub struct SumQuery<T>(pub T);

    impl<T: Add<Output = T> + Clone> Query for SumQuery<T> {
        fn query(&self, other: &Self) -> Self {
            Self(self.0.clone() + other.0.clone())
        }
    }

    impl<T: Add<Output = T> + Sub<Output = T> + Clone> RevQuery for SumQuery<T> {
        fn rev_query(&self, other: &Self) -> Self {
            Self(self.0.clone() - other.0.clone())
        }
    }

    impl<Q1: Query, Q2: Query> Query for (Q1, Q2) {
        fn query(&self, other: &Self) -> Self {
            (self.0.query(&other.0), self.1.query(&other.1))
        }
    }

    impl<Q1: RevQuery, Q2: RevQuery> RevQuery for (Q1, Q2) {
        fn rev_query(&self, other: &Self) -> Self {
            (self.0.rev_query(&other.0), self.1.rev_query(&other.1))
        }
    }

    impl Query for () {
        fn query(&self, _other: &Self) -> Self {}
    }

    impl<T> Unionfind<T> {
        pub fn new(data: Vec<T>) -> Self {
            let size = data.len();
            Self {
                uf: (0..size).collect(),
                size: vec![1; size],
                query: data,
            }
        }

        pub fn len(&self) -> usize {
            self.uf.len()
        }

        pub fn is_empty(&self) -> bool {
            self.len() == 0
        }
    }

    impl<T: Query> Unionfind<T> {
        pub fn unite(&mut self, i: usize, j: usize) -> bool {
            let root_i = self.find_rc(i);
            let root_j = self.find_rc(j);
            if root_i != root_j {
                let size_i = self.size[root_i];
                let size_j = self.size[root_j];
                if size_i > size_j {
                    self.uf[root_j] = root_i;
                    self.size[root_i] = size_i + size_j;
                    let new_data = self.query[root_i].query(&self.query[root_j]);
                    self.query[root_i] = new_data;
                } else {
                    self.uf[root_i] = root_j;
                    self.size[root_j] = size_i + size_j;
                    let new_data = self.query[root_j].query(&self.query[root_i]);
                    self.query[root_j] = new_data;
                }
                true
            } else {
                false
            }
        }

        pub fn find(&self, mut i: usize) -> usize {
            let mut p = self.uf[i];
            while p != i {
                i = p;
                p = self.uf[i];
            }
            p
        }

        pub fn query(&self, i: usize) -> &T {
            &self.query[self.find(i)]
        }

        pub fn find_rc(&mut self, mut i: usize) -> usize {
            let mut p = self.uf[i];
            let mut prev_i = usize::MAX;
            while p != i {
                self.size[i] = prev_i;
                prev_i = i;
                i = p;
                p = self.uf[i];
            }
            while prev_i < self.uf.len() {
                self.uf[prev_i] = p;
                prev_i = self.size[prev_i];
            }
            p
        }

        pub fn query_rc(&mut self, i: usize) -> &T {
            let root = self.find_rc(i);
            &self.query[root]
        }

        pub fn size(&self, i: usize) -> usize {
            let root = self.find(i);
            self.size[root]
        }

        pub fn size_rc(&mut self, i: usize) -> usize {
            let root = self.find_rc(i);
            self.size[root]
        }
    }
}
