use proconio::input;

fn main() {
    input! {
        n: usize,
        c: u64,
    }
    let mut itemss = [vec![], vec![], vec![]];
    let mut cnt = [0; 3];
    for _ in 0..n {
        input! { w: usize, v: u64, k: u64 }
        itemss[w - 1].push((v, k));
        cnt[w - 1] += k;
    }
    if cnt[0] + cnt[1] * 2 + cnt[2] * 3 <= c {
        let mut ans = 0;
        for items in &itemss {
            ans += items.iter().map(|&(vi, ki)| vi * ki).sum::<u64>();
        }
        println!("{ans}");
        return;
    }
    for items in &mut itemss {
        items.sort_unstable_by_key(|x| x.0);
    }
    // eprintln!("{itemss:?}");
    let mut ans = 0u64;
    for r1 in 0..6 {
        if cnt[0] < r1 { continue }
        for r2 in 0..3 {
            if cnt[1] < r2 { continue }
            for r3 in 0..2 {
                if cnt[2] < r3 { continue }
                if r1 + r2 * 2 + r3 * 3 > c { continue; }
                let r = [r1, r2, r3];
                let mut itemss = itemss.clone();
                let mut cnt = cnt;
                let mut sum = 0;
                for i in 0..3 {
                    for _ in 0..r[i] {
                        let last = itemss[i].last_mut().unwrap();
                        sum += last.0;
                        last.1 -= 1;
                        if last.1 == 0 {
                            itemss[i].pop().unwrap();
                        }
                    }
                    cnt[i] -= r[i];
                }
                // eprintln!("sum: {sum}");
                let mut select = [Option::<(u64, u64)>::None; 3];
                for i in 0..3 {
                    let c = 6 / (i + 1) as u64;
                    if cnt[i] < c { continue }
                    if let Some(&(v, k)) = itemss[i].last() && k >= c {
                        select[i] = Some((v * c, k / c));
                        if k.is_multiple_of(c) {
                            itemss[i].pop().unwrap();
                        } else {
                            itemss[i].last_mut().unwrap().1 = k % c;
                        }
                        cnt[i] = cnt[i].checked_sub(k - k % c).unwrap();
                    } else {
                        let mut sum = 0;
                        for _ in 0..c {
                            let last = itemss[i].last_mut().unwrap();
                            sum += last.0;
                            last.1 -= 1;
                            if last.1 == 0 {
                                itemss[i].pop().unwrap();
                            }
                        }
                        cnt[i] = cnt[i].checked_sub(c).unwrap();
                        select[i] = Some((sum, 1));
                    }
                }
                let mut cur = r1 + 2 * r2 + 3 * r3;
                while cur < c {
                    let mut mxi = Option::<usize>::None;
                    for i in 0..3 {
                        if let Some((v, _)) = select[i] && mxi.is_none_or(|i| select[i].unwrap().0 < v) {
                            mxi = Some(i);
                        }
                    }
                    // eprintln!("({r1} {r2} {r3}): {mxi:?}");
                    // eprintln!("{select:?}");
                    let Some(mxi) = mxi else { break };
                    let (v, mxc) = select[mxi].unwrap();
                    if cur + mxc * 6 >= c {
                        sum += v * ((c - cur) / 6);
                        // cur += (c - cur) / 6 * 6;
                        break;
                    }
                    sum += v * mxc;
                    cur += mxc * 6;
                    {
                        select[mxi] = None;
                        let c = 6 / (mxi + 1) as u64;
                        if cnt[mxi] < c { continue }
                        if let Some(&(v, k)) = itemss[mxi].last() && k >= c {
                            select[mxi] = Some((v * c, k / c));
                            if k.is_multiple_of(c) {
                                itemss[mxi].pop().unwrap();
                            } else {
                                itemss[mxi].last_mut().unwrap().1 = k % c;
                            }
                            cnt[mxi] = cnt[mxi].checked_sub(k - k % c).unwrap();
                        } else {
                            let mut sum = 0;
                            for _ in 0..c {
                                let last = itemss[mxi].last_mut().unwrap();
                                sum += last.0;
                                last.1 -= 1;
                                if last.1 == 0 {
                                    itemss[mxi].pop().unwrap();
                                }
                            }
                            cnt[mxi] = cnt[mxi].checked_sub(c).unwrap();
                            select[mxi] = Some((sum, 1));
                        }
                    }
                }
                ans = ans.max(sum);
            }
        }
    }
    println!("{ans}");
}