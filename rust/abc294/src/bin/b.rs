use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        mut a: [[u8; w]; h],
    }

    a.iter_mut().for_each(|ai| {
        ai.iter_mut().for_each(|aij| {
            if *aij == 0 {
                *aij = b'.';
            } else {
                *aij += b'A' - 1;
            }
        })
    });
    for ai in &a {
        println!("{}", std::str::from_utf8(ai).unwrap());
    }
}
