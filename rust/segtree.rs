pub mod segtree {
    pub mod operation {
        use std::{marker::PhantomData, ops};

        pub trait Operator {
            type Query;
            const IDENT: Self::Query;
            fn op(&self, a: &Self::Query, b: &Self::Query) -> Self::Query;
            fn op_assign_left(&self, a: &mut Self::Query, b: &Self::Query) {
                *a = self.op(a, b);
            }
            fn op_assign_right(&self, a: &Self::Query, b: &mut Self::Query) {
                *b = self.op(a, b);
            }
        }

        /// An operator is idempotent if `op(a, a) = a` for all `a`.
        pub trait Idempotent: Operator {}

        trait HasZero {
            const ZERO: Self;
        }

        trait HasOne {
            const ONE: Self;
        }

        trait HasMax {
            const MAX: Self;
        }

        trait HasMin {
            const MIN: Self;
        }

        macro_rules! impl_trait_integer {
    ($($t:ty),*) => {
        $(
            impl HasZero for $t {
                const ZERO: Self = 0;
            }
            impl HasOne for $t {
                const ONE: Self = 1;
            }
            impl HasMax for $t {
                const MAX: Self = <$t>::MAX;
            }
            impl HasMin for $t {
                const MIN: Self = <$t>::MIN;
            }
        )*
    };
}

        impl_trait_integer!(i8, i16, i32, i64, i128, isize);
        impl_trait_integer!(u8, u16, u32, u64, u128, usize);

        macro_rules! impl_auto_trait_for_marker {
    ($t:ident, $($u:ty),*) => {
        $(
            impl<$t> Clone for $u {
                fn clone(&self) -> Self {
                    *self
                }
            }

            impl<$t> Copy for $u {}

            impl<$t> Default for $u {
                fn default() -> Self {
                    Self(PhantomData)
                }
            }

            impl<$t> PartialEq for $u {
                fn eq(&self, _: &Self) -> bool {
                    true
                }
            }

            impl<$t> Eq for $u {}

            impl<$t> ::core::fmt::Debug for $u {
                fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
                    write!(f, "{}", ::core::any::type_name::<$u>())
                }
            }
        )*
    }
}

        // #[derive(Debug)]
        pub struct Add<T>(PhantomData<fn() -> T>);
        pub struct Mul<T>(PhantomData<fn() -> T>);

        impl<T> Operator for Add<T>
        where
            T: ops::Add<Output = T> + Clone + HasZero,
        {
            type Query = T;
            const IDENT: Self::Query = T::ZERO;
            fn op(&self, a: &Self::Query, b: &Self::Query) -> Self::Query {
                a.clone() + b.clone()
            }
        }

        impl<T> Operator for Mul<T>
        where
            T: ops::Mul<Output = T> + Clone + HasOne,
        {
            type Query = T;
            const IDENT: Self::Query = T::ONE;
            fn op(&self, a: &Self::Query, b: &Self::Query) -> Self::Query {
                a.clone() * b.clone()
            }
        }
        pub struct Max<T>(PhantomData<fn() -> T>);
        pub struct Min<T>(PhantomData<fn() -> T>);

        pub const fn max<T>() -> Max<T> {
            Max(PhantomData)
        }
        pub const fn min<T>() -> Min<T> {
            Min(PhantomData)
        }

        impl<T> Operator for Max<T>
        where
            T: Ord + Clone + HasMin,
        {
            type Query = T;
            const IDENT: Self::Query = T::MIN;
            fn op(&self, a: &Self::Query, b: &Self::Query) -> Self::Query {
                if a > b { a.clone() } else { b.clone() }
            }
        }

        impl<T> Idempotent for Max<T> where Max<T>: Operator {}

        impl<T> Operator for Min<T>
        where
            T: Ord + Clone + HasMax,
        {
            type Query = T;
            const IDENT: Self::Query = T::MAX;
            fn op(&self, a: &Self::Query, b: &Self::Query) -> Self::Query {
                if a < b { a.clone() } else { b.clone() }
            }
        }

        impl<T> Idempotent for Min<T> where Min<T>: Operator {}

        impl<T: Operator> Operator for &T {
            type Query = T::Query;
            const IDENT: Self::Query = T::IDENT;
            fn op(&self, a: &Self::Query, b: &Self::Query) -> Self::Query {
                T::op(self, a, b)
            }
        }

        impl_auto_trait_for_marker!(T, Add<T>, Mul<T>, Max<T>, Min<T>);
    }
    use operation::{Idempotent, Operator};
    use std::{
        cmp::Ordering,
        iter,
        ops::{Bound, Deref, DerefMut, RangeBounds},
    };

    #[derive(Debug, Clone)]
    pub struct Segtree<T, OP> {
        len: usize,
        data: Box<[T]>,
        op: OP,
    }

    impl<T, OP> Segtree<T, OP> {
        fn new_empty(op: OP) -> Self {
            Self {
                len: 0,
                data: Box::new([]),
                op,
            }
        }

        pub fn into_boxed_slice(self) -> Box<[T]> {
            self.data
        }

        pub fn into_vec(self) -> Vec<T> {
            self.data.into_vec()
        }
    }

    impl<T, OP> Deref for Segtree<T, OP> {
        type Target = [T];
        fn deref(&self) -> &Self::Target {
            &self.data[self.len..]
        }
    }

    impl<T, OP: Operator<Query = T>> Segtree<T, OP> {
        fn get_unchecked(&self, index: usize) -> &T {
            unsafe { self.data.get_unchecked(index) }
        }

        fn get_unchecked_mut(&mut self, index: usize) -> &mut T {
            unsafe { self.data.get_unchecked_mut(index) }
        }

        fn eval(mut self) -> Self {
            for i in (1..self.len).rev() {
                let new_val = self
                    .op
                    .op(self.get_unchecked(i * 2), self.get_unchecked(i * 2 + 1));
                *self.get_unchecked_mut(i) = new_val;
            }
            self
        }

        pub fn from_iter_op<I: IntoIterator<Item = T>>(iter: I, op: OP) -> Self {
            let iter = iter.into_iter();
            let (size_min, size_max) = iter.size_hint();
            if size_max == Some(0) {
                Self::new_empty(op)
            } else {
                assert_ne!(size_min, 0);
                let half_len_min = size_min.next_power_of_two();
                let half_len_max = size_max.map(usize::next_power_of_two);
                let uninit = if Some(half_len_min) == half_len_max {
                    let half_len = half_len_min;
                    let data = iter::repeat_with(|| OP::IDENT)
                        .take(half_len)
                        .chain(iter.chain(iter::repeat_with(|| OP::IDENT)).take(half_len))
                        .collect();

                    Self {
                        len: half_len,
                        data,
                        op,
                    }
                } else {
                    let data = iter.collect::<Vec<_>>();
                    let half_len = data.len().next_power_of_two();
                    let data = iter::repeat_with(|| OP::IDENT)
                        .take(half_len)
                        .chain(
                            data.into_iter()
                                .chain(iter::repeat_with(|| OP::IDENT))
                                .take(half_len),
                        )
                        .collect();
                    Self {
                        len: half_len,
                        data,
                        op,
                    }
                };
                uninit.eval()
            }
        }

        pub fn len(&self) -> usize {
            self.len
        }

        pub fn is_empty(&self) -> bool {
            self.len == 0
        }

        /// 戻り値を`(l, r)`とすると以下が保証される。
        ///
        /// * `l <= r <= self.len()`
        fn get_lr<R: RangeBounds<usize>>(&self, range: R) -> (usize, usize) {
            use Bound::*;
            let size = self.len;
            let l = match range.start_bound() {
                Excluded(s) => s
                    .checked_add(1)
                    .unwrap_or_else(|| panic!("attempted to index slice from after maximum usize")),
                Included(s) => *s,
                Unbounded => 0,
            };
            let r = match range.end_bound() {
                Excluded(e) => *e,
                Included(e) => e
                    .checked_add(1)
                    .unwrap_or_else(|| panic!("attempted to index slice up to maximum usize")),
                Unbounded => size,
            };
            if l > r {
                panic!("slice index starts at {l} but ends at {r}");
            } else if r > size {
                panic!("range end index {r} out of range for slice of length {size}");
            }
            (l, r)
        }

        pub fn query<R: RangeBounds<usize>>(&self, range: R) -> T {
            let (mut l, mut r) = self.get_lr(range);
            l += self.len;
            r += self.len;
            let mut query_l = OP::IDENT;
            let mut query_r = OP::IDENT;
            while l < r {
                if r & 1 == 1 {
                    r -= 1;
                    self.op.op_assign_right(self.get_unchecked(r), &mut query_r);
                }
                if l & 1 == 1 {
                    self.op.op_assign_left(&mut query_l, self.get_unchecked(l));
                    l += 1;
                }
                l >>= 1;
                r >>= 1;
            }
            self.op.op_assign_left(&mut query_l, &query_r);
            query_l
        }

        pub fn get_mut(&mut self, index: usize) -> ValMut<'_, T, OP> {
            assert!(index < self.len, "index out of bounds");
            ValMut {
                index: index + self.len,
                segtree: self,
            }
        }

        fn update_val(&mut self, mut i: usize) {
            while i > 1 {
                i >>= 1;
                let new_val = self
                    .op
                    .op(self.get_unchecked(i * 2), self.get_unchecked(i * 2 + 1));
                *self.get_unchecked_mut(i) = new_val;
            }
        }

        pub fn update(&mut self, index: usize, value: T) {
            let i = index + self.len;
            *self.get_unchecked_mut(i) = value;
            self.update_val(i);
        }

        /// `pred(self.query(l..j))`が`true`となる最大の`j`をO(log(n))で求める。
        pub fn upper_bound<P>(&self, l: usize, mut pred: P) -> usize
        where
            P: FnMut(&T) -> bool,
        {
            match l.cmp(&self.len()) {
                Ordering::Equal => return l,
                Ordering::Greater => {
                    panic!("index {l} out of range for slice of length {}", self.len())
                }
                _ => {}
            };
            let stop = self.data.len() / ((self.len - l + 1).next_power_of_two() >> 1) - 1;
            let mut l = l + self.len;
            let mut l_query = OP::IDENT;
            loop {
                while l & 1 == 0 {
                    l >>= 1;
                }
                let next_query = self.op.op(&l_query, self.get_unchecked(l));
                if pred(&next_query) {
                    l_query = next_query;
                } else {
                    break;
                }
                if l == stop {
                    return self.len;
                }
                l = (l >> 1) + 1;
            }
            while l < self.len {
                l <<= 1;
                let next_query = self.op.op(&l_query, self.get_unchecked(l));
                if pred(&next_query) {
                    l_query = next_query;
                    l += 1;
                }
            }
            l - self.len
        }

        // TODO: Verify this implementation
        /// `pred(self.query(j..r))`が`true`となる最小の`j`をO(log(n))で求める。
        pub fn lower_bound<P>(&self, r: usize, mut pred: P) -> usize
        where
            P: FnMut(&T) -> bool,
        {
            if r > self.len {
                panic!("index {r} out of range for slice of length {}", self.len())
            }
            if r == 0 {
                return 0;
            }
            let stop = self.len >> r.ilog2();
            let mut r = r + self.len - 1;
            let mut r_query = OP::IDENT;
            loop {
                while r & 1 == 1 {
                    r >>= 1;
                }
                if r == 0 {
                    r = 1;
                }
                let next_query = self.op.op(&self.data[r], &r_query);
                if pred(&next_query) {
                    r_query = next_query;
                } else {
                    break;
                }
                if r == stop {
                    return 0;
                }
                r = (r >> 1) - 1;
            }
            while r < self.len {
                r = (r << 1) + 1;
                let next_query = self.op.op(&self.data[r], &r_query);
                if pred(&next_query) {
                    r_query = next_query;
                    r -= 1;
                }
            }
            r + 1 - self.len
        }
    }

    impl<T, OP: Idempotent<Query = T>> Segtree<T, OP> {
        pub fn fill(&mut self, value: T)
        where
            T: Clone,
        {
            if !self.data.is_empty() {
                self.data[1..].fill(value);
            }
        }

        /// fは呼ばれるたびに同じ値を返す必要がある。そうでない場合、セグメント木の性質が壊れる。
        pub fn fill_with<F: FnMut() -> T>(&mut self, f: F) {
            if !self.data.is_empty() {
                self.data[1..].fill_with(f);
            }
        }
    }

    pub struct ValMut<'a, T, OP: Operator<Query = T>> {
        segtree: &'a mut Segtree<T, OP>,
        index: usize,
    }

    impl<T, OP: Operator<Query = T>> Deref for ValMut<'_, T, OP> {
        type Target = T;
        fn deref(&self) -> &Self::Target {
            &self.segtree.data[self.index]
        }
    }

    impl<T, OP: Operator<Query = T>> DerefMut for ValMut<'_, T, OP> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.segtree.data[self.index]
        }
    }

    impl<T, OP: Operator<Query = T>> Drop for ValMut<'_, T, OP> {
        fn drop(&mut self) {
            self.segtree.update_val(self.index);
        }
    }

    impl<I, OP> FromIterator<I> for Segtree<I, OP>
    where
        OP: Default + Operator<Query = I>,
    {
        fn from_iter<T: IntoIterator<Item = I>>(iter: T) -> Self {
            Self::from_iter_op(iter, OP::default())
        }
    }

    impl<T, OP> From<Segtree<T, OP>> for Box<[T]> {
        fn from(value: Segtree<T, OP>) -> Self {
            value.into_boxed_slice()
        }
    }

    impl<T, OP> From<Segtree<T, OP>> for Vec<T> {
        fn from(value: Segtree<T, OP>) -> Self {
            value.into_vec()
        }
    }
}
