use proconio::input;

fn main() {
    input! {
        d: u32,
        mut f: u32,
    }
    while f <= d {
        f += 7;
    }
    println!("{}", f - d);
}
