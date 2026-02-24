use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [[i32; n]; n],
    }

    let mut ans = vec![false; n];
    let mut score = a.iter().map(|ai| ai.iter().sum::<i32>()).collect::<Vec<_>>();

    while let Some((i, &s)) = score.iter().enumerate().find(|&(_, &x)| x < 0) {
        for j in 0..n {
            if i == j { continue }
            if ans[i] == ans[j] {
                score[j] -= a[i][j] * 2;
            } else {
                score[j] += a[i][j] * 2;
            }
        }
        ans[i] ^= true;
        score[i] = -s;
        // for i in 0..n {
        //     let mut s = 0;
        //     for j in 0..n {
        //         if ans[i] == ans[j] {
        //             s += a[i][j];
        //         } else {
        //             s -= a[i][j];
        //         }
        //     }
        //     assert_eq!(s, score[i]);
        // }
    }

    ans.iter().for_each(|&f| print!("{}", if f { 'X' } else { 'Y' }));
    println!();
}