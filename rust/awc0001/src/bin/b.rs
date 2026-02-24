fn main() {
    proconio::input! {
        n: usize,
        l: u32,
        r: u32,
        p: [u32; n],
    }
    let Some((ans, _)) = p.iter().enumerate().rev().filter(|&(_, &pi)| (l..=r).contains(&pi)).max_by_key(|&(_, &pi)| pi) else {
        println!("-1");
        return;
    };
    println!("{}", ans + 1);
    
}