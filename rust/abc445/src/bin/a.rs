fn main() {
    proconio::input! {
        s: proconio::marker::Bytes,
    }
    if s.first().unwrap() == s.last().unwrap() {
        println!("Yes");
    } else {
        println!("No");
    }
}