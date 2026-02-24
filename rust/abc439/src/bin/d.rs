use std::collections::HashMap;

fn main() {
    proconio::input! {
        n: usize,
        a: [u32; n],
    }

    let mut s1 = HashMap::new();
    let mut s2 = HashMap::new();
    for &ai in &a {
        *s2.entry(ai).or_insert(0u32) += 1;
    }

    let mut ans = 0;
    for &ai in &a {
        *s2.get_mut(&ai).unwrap() -= 1;
        if ai.is_multiple_of(5) {
            let k = ai / 5;
            ans += s1.get(&(k * 3)).copied().unwrap_or_default() as u64 * s1.get(&(k * 7)).copied().unwrap_or_default() as u64;
            ans += s2.get(&(k * 3)).copied().unwrap_or_default() as u64 * s2.get(&(k * 7)).copied().unwrap_or_default() as u64;
        }
        *s1.entry(ai).or_insert(0u32) += 1;
    }
    println!("{ans}");
}