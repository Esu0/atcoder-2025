#[allow(unused_imports)]
use proconio::{input, marker};

fn main() {
    input! {
        n: usize,
        ab: [(i32, i32); n],
    }

    let mut exists = vec![false; n * 2];
    for &(ai, bi) in &ab {
        if bi == 1 {
            println!("No");
            return;
        }
        if ai != -1 && bi != -1 && ai >= bi {
            println!("No");
            return;
        }
        if ai != -1 {
            let ai = ai as usize;
            if exists[ai - 1] {
                println!("No");
                return;
            }
            exists[ai - 1] = true;
        }
        if bi != -1 {
            let bi = bi as usize;
            if exists[bi - 1] {
                println!("No");
                return;
            }
            exists[bi - 1] = true;
        }
    }

}
