use proconio::input;

#[proconio::fastout]
fn main() {
    input! {
        t: usize,
    }
    let mut buf = vec![];
    for _ in 0..t {
        buf.clear();
        input! { q: u8, l: u64, r: u64 }
        let ans = if l % 2 == 1 && r % 2 == 0 {
            let pl = l.count_ones() % 2;
            let pr = r.count_ones() % 2;
            let k = 63 - (l ^ r).leading_zeros();
            if pl == pr {
                if q == 0 {
                    (r - l + 1) / 2 + 1
                } else {
                    for i in l..=r {
                        print!("{}", if i.count_ones() % 2 == pl { '1' } else { '0' });
                    }
                    println!();
                    continue;
                }
            } else if l + (1 << k) <= r {
                if q == 0 {
                    (r - l + 1) / 2
                } else {
                    for i in l..=r {
                        print!("{}", if i.count_ones() % 2 == pl { '1' } else { '0' });
                    }
                    println!();
                    continue;
                }
            } else {
                if q == 0 {
                    (r - l + 1) / 2 + 1
                } else {
                    for i in l..=r {
                        let pi = i.count_ones() % 2;
                        let f = if (i >> k) & 1 == 0 { pl == pi } else { pr == pi };
                        print!("{}", if f { '1' } else { '0' });
                    }
                    println!();
                    continue;
                }
            }
        } else {
            if q == 0 {
                (r - l + 1).div_ceil(2)
            } else {
                let mut cnt = 0;
                for i in l..=r {
                    if i.count_ones() % 2 == 0 {
                        buf.push(1);
                        cnt += 1;
                    } else {
                        buf.push(0);
                    }
                }
                if (r - l + 1) - cnt > cnt {
                    buf.iter_mut().for_each(|bi| *bi ^= 1);
                }
                buf.iter_mut().for_each(|bi| *bi += b'0');
                println!("{}", std::str::from_utf8(&buf).unwrap());
                continue;
            }
        };
        println!("{ans}");
    }
}