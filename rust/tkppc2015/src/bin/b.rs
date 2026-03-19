use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [u32; n],
    }
    let (ans, _) = a.iter().enumerate().rev().max_by_key(|&(_, &v)| v).unwrap();
    println!("{}", ans + 1);
}
