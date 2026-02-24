fn main() {
    proconio::input! {
        n: usize,
        m: usize,
        a: [u32; n],
        b: [u32; m],
    }
    let mut i = 0;
    let mut j = 0;
    let mut k = 1;
    let mut ansa = vec![0; n];
    let mut ansb = vec![0; m];
    loop {
        if i == n {
            if j == m {
                break;
            }
            ansb[j] = k;
            j += 1;
            k += 1;
            continue;
        }
        if j == m {
            ansa[i] = k;
            k += 1;
            i += 1;
            continue;
        }
        if a[i] < b[j] {
            ansa[i] = k;
            k += 1;
            i += 1;
        } else {
            ansb[j] = k;
            k += 1;
            j += 1;
        }
    }
    ansa.iter().for_each(|&ai| print!("{ai} "));
    println!();
    ansb.iter().for_each(|&bi| print!("{bi} "));
    println!();
}
