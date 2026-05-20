#[allow(unused_imports)]
use proconio::{input, marker};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [marker::Bytes; n],
    }

    if m > 15 {
        println!("Yes");
        let mut used = vec![false; n];
        let mut ans = vec![b'0'; m];
        for i in 0..m {
            let mut cnt = [0u32; 2];
            for (j, sj) in s.iter().enumerate() {
                if used[j] { continue }
                cnt[(sj[i] - b'0') as usize] += 1;
            }
            if cnt[0] + cnt[1] == 0 { break }
            if cnt[0] < cnt[1] {
                ans[i] = b'1';
            }
            for (j, sj) in s.iter().enumerate() {
                if sj[i] == ans[i] { used[j] = true; }
            }
        }
        assert!(used.iter().copied().all(|x| x));
        println!("{}", std::str::from_utf8(&ans).unwrap());
    } else {
        let mut ok = vec![true; 1 << m];
        for si in &s {
            let mut b = 0;
            for &d in si.iter() {
                b = b * 2 + (1 - (d - b'0') as u32);
            }
            ok[b as usize] = false;
        }
        let ans = ok.iter().enumerate().find(|&(_, &k)| k);
        if let Some((ans, _)) = ans {
            println!("Yes");
            for i in (0..m).rev() {
                print!("{}", (ans >> i) & 1);
            }
            println!();
        } else {
            println!("No");
        }
    }
}
