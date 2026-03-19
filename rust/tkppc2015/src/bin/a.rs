use proconio::input;

#[proconio::fastout]
fn main() {
    input! { n: usize }
    for _ in 0..n {
        input! { a: u32, b: u32 }
        println!("{}", a + b);
    }
}
