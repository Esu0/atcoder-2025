fn main() {
    proconio::input! {
        n: usize,
        mut a: [u32; n],
    }
    a.iter()
        .filter(|&&ai| ai % 2 == 0)
        .for_each(|&ai| print!("{ai} "));
    println!();
}
