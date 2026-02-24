fn main() {
    proconio::input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }
    let Some((ans, _)) = a.iter().enumerate().find(|&(_, &ai)| ai == k) else {
        println!("-1");
        return;
    };
    println!("{}", ans + 1);
}