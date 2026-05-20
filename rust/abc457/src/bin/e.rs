use std::collections::{BTreeSet, HashMap};

#[allow(unused_imports)]
use proconio::{input, marker};

fn main() {
    input! {
        n: usize,
        m: usize,
        mut lr: [(usize, usize); m],
        q: usize,
        st: [(usize, usize); q],
    }

    lr.iter_mut().for_each(|lri| lri.0 -= 1);
    let mut ls = vec![BTreeSet::new(); n + 1];
    let mut rs = vec![BTreeSet::new(); n];
    for &(li, ri) in &lr {
        ls[ri].insert(li);
        rs[li].insert(ri);
    }
    let mut lr_set = HashMap::<(usize, usize), usize>::new();
    for &(li, ri) in &lr {
        *lr_set.entry((li, ri)).or_insert(0) += 1;
    }

    let mut ans = vec![false; q];
    lr.sort_unstable_by_key(|x| (x.1, std::cmp::Reverse(x.0)));
    let mut maxl: i32 = -1;
    let mut j = 0;
    let mut ist = st.iter().enumerate().map(|(i, &(si, ti))| (i, si - 1, ti)).collect::<Vec<_>>();
    ist.sort_unstable_by_key(|x| (x.2, std::cmp::Reverse(x.1)));
    for &(i, si, ti) in &ist {
        while j < lr.len() && (lr[j].1 < ti || lr[j].1 == ti && lr[j].0 > si) {
            maxl = maxl.max(lr[j].0 as i32);
            j += 1;
        }
        // dbg!(maxl);
        let a = rs[si].range(..=ti).next_back().copied();
        let b = ls[ti].range(si..).next().copied();
        if a == Some(ti) {
            ans[i] = maxl >= si as i32 || *lr_set.get(&(si, ti)).unwrap() >= 2;
        } else {
            ans[i] = a.is_some_and(|x| b.is_some_and(|y| y <= x));
        }
    }

    for &a in &ans {
        if a {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
