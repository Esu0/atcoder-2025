#[proconio::fastout]
fn main() {
    proconio::input! {
        t: usize,
    }

    for _ in 0..t {
        proconio::input! {
            n: u32,
            s: proconio::marker::Bytes,
        }
        let mut vis = vec![false; s.len() + 1];
        let mut stack = Vec::with_capacity(s.len() + 1);
        stack.push(0);
        vis[0] = true;
        while let Some(u) = stack.pop() {
            for i in 0..n {
                if u & (1 << i) == 0 {
                    let v = u | (1 << i);
                    if !vis[v] && s[v - 1] == b'0' {
                        stack.push(v);
                        vis[v] = true;
                    }
                }
            }
        }
        if *vis.last().unwrap() {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}