use std::collections::HashSet;

use rand::RngExt;

fn main() {
    let n = 60000;
    println!("{n}");
    let mut set = HashSet::<(u32, u32)>::new();
    let mut rng = rand::rng();
    for _ in 0..n {
        let x = rng.random_range(0u32..=20_000_000);
        let y = rng.random_range(0u32..=20_000_000);
        if set.contains(&(x, y)) {
            continue;
        }
        set.insert((x, y));
        println!("{x} {y}");
    }
}