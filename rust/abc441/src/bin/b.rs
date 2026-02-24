use proconio::{input, marker};
fn main() {
    input! {
        _: usize,
        _: usize,
        s: marker::Bytes,
        t: marker::Bytes,
        q: usize,
        w: [marker::Bytes; q],
    }

    for wi in &w {
        let a = wi.iter().all(|&ci| s.contains(&ci));
        let b = wi.iter().all(|&ci| t.contains(&ci));
        if a && b {
            println!("Unknown");
        } else if a {
            println!("Takahashi");
        } else {
            assert!(b);
            println!("Aoki");
        }
    }
}