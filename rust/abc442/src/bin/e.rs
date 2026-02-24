use proconio::input;
#[derive(Debug, Clone, Copy)]
struct Point(i64, i64);

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        let s1 = self.shougen();
        let s2 = other.shougen();
        // let mul = if s2 == 3 || s2 == 2 { -1 } else { 1 };
        (s1, self.1 * other.0).cmp(&(s2, other.1 * self.0)).is_eq()
    }
}
impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let s1 = self.shougen();
        let s2 = other.shougen();
        // let mul = if s2 == 3 || s2 == 2 { -1 } else { 1 };
        Some((s1, self.1 * other.0).cmp(&(s2, other.1 * self.0)))
    }
}

impl Point {
    fn shougen(self) -> u8 {
        let Self(xi, yi) = self;
        if xi > 0 && yi >= 0 {
            0
        } else if xi <= 0 && yi > 0 {
            1
        } else if xi < 0 && yi <= 0 {
            2
        } else {
            3
        }
    }
}


fn main() {
    input! {
        n: usize,
        q: usize,
        xy: [(i64, i64); n],
        ab: [(usize, usize); q],
    }

    let mut points = xy.iter().copied().map(|(xi, yi)| Point(xi, yi)).collect::<Vec<_>>();
    // for &p in &points {
    //     eprintln!("{p:?}: {}", p.shougen());
    // }
    points.sort_unstable_by(|&p1, &p2| p1.partial_cmp(&p2).unwrap());
    eprintln!("{points:?}");
    for &(ai, bi) in &ab {
        let p1 = Point(xy[ai - 1].0, xy[ai - 1].1);
        let p2 = Point(xy[bi - 1].0, xy[bi - 1].1);
        let k = points.partition_point(|&pi| pi < p2);
        let l = points.partition_point(|&pi| pi <= p1);
        // eprintln!("{k} {l}");
        let ans = if l > k {
            l - k
        } else if l == k {
            n
        } else {
            l + n - k
        };
        println!("{}", ans);
    }
}