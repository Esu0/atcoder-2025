use proconio::input;

fn main() {
    input! {
        t: usize,
    }
    for _ in 0..t {
        input! {
            n: usize,
            k: usize,
            l: usize,
            mut a: [usize; n],
        }
        a.sort_unstable();
        let mx = *a.last().unwrap();
        let mn = a[0];
        let mut edge_sum = l - mx + mn;
        let mut edge_min = (l - mx).min(mn);
        for i in (0..n - 1).rev() {
            a[i + 1] -= a[i];
        }
        a[1..].sort_unstable_by_key(|&k| std::cmp::Reverse(k));
        let mut ans = k * edge_sum - edge_min;
        let mut sum = 0;
        for i in 1..=k.min(n - 1) {
            sum += a[i] / 2;
            edge_sum += a[i];
            edge_min += a[i] / 2;
            let mut x = sum;
            if i < k {
                x += edge_sum * (k - i) - edge_min;
            }
            ans = ans.max(x);
        }
        println!("{ans}");
    }
}
