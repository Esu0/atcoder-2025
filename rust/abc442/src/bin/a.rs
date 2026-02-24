use proconio::input;

fn main() {
    input! {
        s: proconio::marker::Bytes,
    }
    println!("{}", s.iter().filter(|&&si| si == b'i' || si == b'j').count());
}