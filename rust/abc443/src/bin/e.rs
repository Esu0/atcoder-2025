use proconio::input;
#[proconio::fastout]
fn main() {
    input! {
        t: usize,
    }
    let mut stack = vec![];
    for _ in 0..t {
        input! {
            n: usize,
            c: usize,
            s: [proconio::marker::Bytes; n],
        }
        let c = c - 1;
        let mut marked_row = vec![Option::<usize>::None; n];
        for (i, si) in s.iter().enumerate().rev() {
            for (j, &sij) in si.iter().enumerate() {
                if sij == b'#' && marked_row[j].is_none() {
                    marked_row[j] = Some(i);
                }
            }
        }

        stack.push((n - 1, c));
        let mut vis = vec![vec![false; n]; n];
        let dj = [usize::MAX, 0, 1];
        vis[n - 1][c] = true;
        while let Some((i, j)) = stack.pop() {
            if i == 0 { continue }
            for dj in dj {
                let ni = i - 1;
                let nj = j.wrapping_sub(dj);
                if (0..n).contains(&nj) && !vis[ni][nj] && (s[ni][nj] == b'.' || marked_row[nj] == Some(ni)) {
                    stack.push((ni, nj));
                    vis[ni][nj] = true;
                }
            }
            if marked_row[j] == Some(i) {
                for ni in 0..i {
                    if !vis[ni][j] {
                        stack.push((ni, j));
                        vis[ni][j] = true;
                    }
                }
            }
        }
        for &vi in &vis[0] {
            let c = if vi { '1' } else { '0' };
            print!("{c}");
        }
        println!();
    }
}