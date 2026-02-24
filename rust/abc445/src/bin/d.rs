use std::collections::{BTreeMap, HashSet};
use std::collections::btree_map::Entry;
fn main() {
    proconio::input! {
        h: u32,
        w: u32,
        n: usize,
        hw: [(u32, u32); n],
    }

    let mut maph = BTreeMap::new();
    let mut mapw = BTreeMap::new();
    for (i, &(hi, wi)) in hw.iter().enumerate() {
        maph.entry(hi).or_insert_with(HashSet::new).insert(i);
        mapw.entry(wi).or_insert_with(HashSet::new).insert(i);
    }

    let mut h = h;
    let mut w = w;
    let mut ans = vec![(u32::MAX, u32::MAX); n];
    while !maph.is_empty() {
        if let Some(entry) = maph.last_entry() && *entry.key() == h {
            for &v in entry.get() {
                w -= hw[v].1;
                ans[v] = (1, w + 1);
                // eprintln!("{v}: ({}, {})", hw[v].0, hw[v].1);
                // eprintln!("h, w: {h} {w}");

                let Entry::Occupied(mut entry) = mapw.entry(hw[v].1) else {
                    unreachable!();
                };
                entry.get_mut().remove(&v);
                if entry.get().is_empty() {
                    entry.remove();
                }
            }
            entry.remove();
        } else {
            let entry = mapw.last_entry().unwrap();
            assert!(entry.key() == &w);
            for &v in entry.get() {
                h -= hw[v].0;
                ans[v] = (h + 1, 1);
                // eprintln!("{v}: ({}, {})", hw[v].0, hw[v].1);
                // eprintln!("h, w: {h} {w}");
                let Entry::Occupied(mut entry) = maph.entry(hw[v].0) else {
                    unreachable!();
                };
                entry.get_mut().remove(&v);
                if entry.get().is_empty() {
                    entry.remove();
                }
            }
            entry.remove();
        }
    }
    assert!(h == 0 || w == 0);
    for &(x, y) in &ans {
        println!("{x} {y}");
    }
}
