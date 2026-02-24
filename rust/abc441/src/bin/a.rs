fn main() {
    proconio::input! {
        p: usize,
        q: usize,
        x: usize,
        y: usize,
    }
    if (p..p+100).contains(&x) && (q..q+100).contains(&y) {
        println!("Yes");
    } else {
        println!("No");
    }
}