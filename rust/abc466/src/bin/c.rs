#[allow(unused_imports)]
use proconio::{input_interactive as input, marker};

fn main() {
    input! {
        n: usize
    }
    let mut ans = 0;
    let mut j = 1;
    for i in 0..n {
        while j < n {
            if i == j {
                j += 1;
                continue;
            }
            println!("? {} {}", i + 1, j + 1);
            input! {s: String}
            if s == "No" {
                break;
            }
            j += 1;
        }
        ans += j - 1 - i;
    }
    println!("! {}", ans);
}
