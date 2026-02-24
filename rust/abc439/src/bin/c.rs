fn main() {
    proconio::input! {
        n: usize,
    }

    let mut cnt = vec![0u8; n + 1];
    let mut x = 1;
    while x * x <= n {
        let mut y = x + 1;
        while x * x + y * y <= n {
            cnt[x * x + y * y] = cnt[x * x + y * y].saturating_add(1);
            y += 1;
        }
        x += 1;
    }
    let ans = cnt.iter().enumerate().filter(|&(_, &ci)| ci == 1).map(|(i, _)| i).collect::<Vec<_>>();
    println!("{}", ans.len());
    for &ai in &ans {
        print!("{ai} ");
    }
    println!();
}