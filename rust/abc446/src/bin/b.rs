use proconio::{input, marker};

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut used = vec![false; m];
    'next: for _ in 0..n {
        input! {
            l: usize,
            x: [usize; l],
        }
        for &xi in &x {
            if !used[xi - 1] {
                used[xi - 1] = true;
                println!("{xi}");
                continue 'next;
            }
        }
        println!("0");
    }
}
