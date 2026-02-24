use std::collections::VecDeque;

use proconio::input_interactive;

fn main() {
    input_interactive! {
        n: u8,
        l: usize,
        r: usize,
    }
    let r = r + 1;
    let sz = 1 << n;
    let mut parent = vec![usize::MAX; sz + 1];
    let mut queue = VecDeque::from([l]);
    parent[l] = l;
    while let Some(u) = queue.pop_front() {
        for i in 0..=u.trailing_zeros().min(n as _) {
            // eprintln!("{u} {i}");
            let nu = u + (1 << i);
            if nu <= sz && parent[nu] == usize::MAX {
                parent[nu] = u;
                queue.push_back(nu);
            }
            if let Some(nu) = u.checked_sub(1 << i) && parent[nu] == usize::MAX {
                parent[nu] = u;
                queue.push_back(nu);
            }
        }
    }

    let mut ans = 0;
    let mut cur = r;
    while cur != l {
        let p = parent[cur];
        let i2 = p.abs_diff(cur);
        assert!(i2.is_power_of_two());
        let i = i2.trailing_zeros();
        let j = p.min(cur) >> i;
        println!("? {i} {j}");
        input_interactive!{ t: u32 }
        if p < cur { ans = (ans + t) % 100; } else { ans = (100 + ans - t) % 100; }
        cur = p;
    }
    println!("! {ans}");
}
