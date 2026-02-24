fn main() {
    proconio::input! {
        mut n: u32,
    }
    for _ in 0..3000 {
        let s = n.to_string();
        n = 0;
        for &c in s.as_bytes() {
            let d = (c - b'0') as u32;
            n += d * d;
        }
        if n == 1 {
            println!("Yes");
            return;
        }
    }
    println!("No");
}