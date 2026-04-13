#[allow(unused_imports)]
use proconio::{input, marker};

fn main() {
    input! {
        n: usize,
    }
    let mut cost = vec![vec![0; n]; n];
    for i in 0..n {
        for j in i + 1..n {
            input! {
                cij: u32,
            }
            cost[i][j] = cij;
        }
    }
    for a in 0..n {
        for b in a + 1..n {
            for c in b + 1..n {
                if cost[a][c] > cost[a][b] + cost[b][c] {
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}
