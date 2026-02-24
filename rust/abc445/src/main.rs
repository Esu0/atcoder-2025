fn main() {
    proconio::input! {
        n: usize,
        s: [proconio::marker::Bytes; n],
    }
    let mut cnt = 0;
    for si in &s {
        for &sij in si {
            if sij == b'o' {
                cnt += 1;
            }
        }
    }
    println!("{cnt}");
}
