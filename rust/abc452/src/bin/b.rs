#[allow(unused_imports)]
use proconio::{input, marker};

fn main() {
    input! {
        h: usize,
        w: usize,
    }
    let mut s = vec![vec![b'.'; w]; h];
    s[0].fill(b'#');
    s.last_mut().unwrap().fill(b'#');
    s.iter_mut().for_each(|si| {
        *si.first_mut().unwrap() = b'#';
        *si.last_mut().unwrap() = b'#';
    });
    s.iter().for_each(|si| println!("{}", std::str::from_utf8(si).unwrap()));
}
