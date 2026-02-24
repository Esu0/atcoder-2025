use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut ans = n;
    let mut stack = Vec::with_capacity(n);
    for i in 0..n {
        stack.push(i);
        while stack.len() >= 4 {
            let ai = a[*stack.last().unwrap()];
            let mut flg = true;
            let l = stack.len();
            for j in 1..4 {
                if ai != a[stack[l - j - 1]] {
                    flg = false;
                }
            }
            if flg {
                for _ in 0..4 {
                    stack.pop().unwrap();
                }
                ans -= 4;
            } else {
                break;
            }
        }
    }
    println!("{ans}");
}
