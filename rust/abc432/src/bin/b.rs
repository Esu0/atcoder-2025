use proconio::input;

fn main() {
    input! {
        x: proconio::marker::Bytes,
    }

    let mut set = [0u32; 10];
    for &xi in &x {
        set[(xi - b'0') as usize] += 1;
    }
    let mut ans = Vec::with_capacity(x.len());
    for (i, si) in set.iter_mut().enumerate().skip(1) {
        if *si > 0 {
            *si -= 1;
            ans.push(i as u8 + b'0');
            break;
        }
    }
    for (i, &si) in set.iter().enumerate() {
        for _ in 0..si {
            ans.push(i as u8 + b'0');
        }
    }
    println!("{}", std::str::from_utf8(&ans).unwrap());
}