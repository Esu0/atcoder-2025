use proconio::input;
#[proconio::fastout]
fn main() {
    input! {
        q: usize,
        a: [u8; q],
    }
    let mut vol = 0u32;
    let mut play = false;
    for &ai in &a {
        if ai == 1 {
            vol += 1;
        } else if ai == 2 {
            vol = vol.saturating_sub(1);
        } else {
            play ^= true;
        }
        if play && vol >= 3 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}