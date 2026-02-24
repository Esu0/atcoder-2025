fn main() {
    proconio::input! {
        n: usize,
        k: usize,
    }
    let mut x = 0;
    for i in n.. {
        x += i;
        if x >= k {
            println!("{}", i - n);
            return;
        }
    }
}