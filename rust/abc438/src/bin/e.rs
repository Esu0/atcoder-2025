use proconio::input;
fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        tb: [(usize, usize); q],
    }

    let mut dob = vec![[(0, 0); 31]; n];
    for (i, &ai) in a.iter().enumerate() {
        dob[i][0] = (ai - 1, i + 1);
    }
    for j in 1..31 {
        for i in 0..n {
            let (to, am) = dob[i][j - 1];
            dob[i][j] = (dob[to][j - 1].0, am + dob[to][j - 1].1);
        }
    }

    for &(ti, bi) in &tb {
        let mut cur = bi - 1;
        let mut ans = 0;
        for j in 0..31 {
            if (ti >> j) & 1 != 0 {
                ans += dob[cur][j].1;
                cur = dob[cur][j].0;
            }
        }
        println!("{ans}");
    }
}
