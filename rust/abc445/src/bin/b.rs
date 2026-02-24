fn main() {
    proconio::input! {
        n: usize,
        s: [String; n],
    }
    let k = s.iter().map(|si| si.len()).max().unwrap();
    for si in s {
        print!("{}", ".".repeat((k - si.len()) / 2));
        print!("{si}");
        println!("{}", ".".repeat((k - si.len()) / 2));
    }
}