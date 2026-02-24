use std::collections::{BTreeMap, btree_map::Entry};

fn main() {
    proconio::input! {
        n: usize,
        k: usize,
        h: [i32; n],
    }

    let mut map = BTreeMap::new();
    for &hi in &h[0..k-1] {
        *map.entry(hi).or_insert(0u32) += 1;
    }
    let mut ans = 0;
    for (&hj, &hi) in h.iter().zip(&h[k-1..]) {
        *map.entry(hi).or_insert(0u32) += 1;
        ans = ans.max(map.last_key_value().unwrap().0 - map.first_key_value().unwrap().0);
        let Entry::Occupied(mut entry) = map.entry(hj) else {
            unreachable!();
        };
        *entry.get_mut() -= 1;
        if *entry.get() == 0 {
            entry.remove();
        }
    }
    println!("{ans}");
}