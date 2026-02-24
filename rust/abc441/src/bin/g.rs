use proconio::input;
use ac_library::{LazySegtree, MapMonoid, Monoid};

struct Op;
impl Monoid for Op {
    type S = (bool, bool, u64);
    fn binary_operation(a: &Self::S, b: &Self::S) -> Self::S {
        (a.0 | b.0, a.1 | b.1, a.2.max(b.2))
    }
    fn identity() -> Self::S {
        (false, false, 0)
    }
}

struct LazyOp;
impl MapMonoid for LazyOp {
    type F = (bool, bool, u64);
    type M = Op;

    fn composition(f: &Self::F, g: &Self::F) -> Self::F {
        (f.0 | g.0, f.1 ^ g.1, if f.0 { f.2 } else { f.2 + g.2 })
    }

    fn mapping(f: &Self::F, x: &<Self::M as Monoid>::S) -> <Self::M as Monoid>::S {
        let mut x = *x;
        if f.0 {
            x.2 = 0;
        }
        if f.1 {
            x = (x.1, x.0, x.2);
        }
        if x.0 {
            x.2 += f.2;
        }
        x
    }

    fn identity_map() -> Self::F {
        (false, false, 0)
    }
}

fn main() {
    input! {
        n: usize,
        q: usize,
    }

    let mut segtree = LazySegtree::<LazyOp>::from(vec![(true, false, 0); n]);
    for _ in 0..q {
        input! {
            t: u8,
            l: usize,
            r: usize,
        }
        if t == 1 {
            input! {
                x: u64,
            }
            segtree.apply_range(l-1..r, (false, false, x));
        } else if t == 2 {
            segtree.apply_range(l-1..r, (true, true, 0));
        } else {
            println!("{}", segtree.prod(l-1..r).2);
        }
    }
}