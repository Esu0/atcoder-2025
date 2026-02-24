fn check(x: usize) -> bool {
    let mut prev = 9;
    let mut x = x;
    while x > 0 {
        if x % 10 > prev {
            return false;
        }
        prev = x % 10;
        x /= 10;
    }
    true
}
fn main() {
    proconio::input! {
        n: usize,
    }

    for i in 1.. {
        // eprintln!("{}", n*i);
        if check(n*i) {
            println!("{}", n * i);
            return;
        }
    }
}