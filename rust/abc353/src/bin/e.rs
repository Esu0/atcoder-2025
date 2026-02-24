#[allow(unused_imports)]
use proconio::{input, marker};

fn main() {
    input! {
        n: usize,
        s: [marker::Bytes; n],
    }

    let mut trie = Vec::with_capacity(300_001);
    let mut dp = Vec::with_capacity(300_001);
    trie.push([usize::MAX; 26]);
    dp.push(0u64);
    for si in &s {
        let mut cur = 0;
        for &c in si {
            dp[cur] += 1;
            let idx = (c - b'a') as usize;
            let mut id = trie[cur][idx];
            if id == usize::MAX {
                id = trie.len();
                trie.push([usize::MAX; 26]);
                dp.push(0);
            }
            trie[cur][idx] = id;
            cur = id;
        }
        dp[cur] += 1;
    }
    // eprintln!("{dp:?}");
    let mut ans = 0;
    for i in 1..trie.len() {
        ans += dp[i] * (dp[i] - 1) / 2;
    }
    println!("{ans}");
}
