use proconio::input;
type MInt = modulo::ModInt<998244353>;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }
    let mut dp: Vec<Vec<(MInt, MInt)>> = vec![vec![]; n + 1];
    let mut diff = MInt::new(0);
    // let mut ans = MInt::new(0);
    for &ai in &a {
        if dp[ai].len() >= ai {
            let (s, u) = dp[ai][dp[ai].len() - ai];
            let nu = s - u + MInt::new(1);
            let ns = diff;
            diff -= s - u + MInt::new(1);
            dp[ai].push((ns, nu));
        } else {
            dp[ai].push((diff, MInt::new(0)));
        }
        if dp[ai].len() >= ai {
            let t = dp[ai][dp[ai].len() - ai];
            diff += t.0 - t.1 + MInt::new(1);
        }
    }
    println!("{diff}");
}
pub mod modulo {
    use std::{
        fmt::{self, Display},
        ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
    };

    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default, PartialOrd, Ord, Hash)]
    pub struct ModInt<const MOD: u32>(u32);

    const fn check_primary<const M: u32>() -> bool {
        match M {
            0 => false,
            1 => false,
            2 => true,
            _ => {
                if M.is_multiple_of(2) {
                    return false;
                }
                let mut i = 3;
                while i * i <= M {
                    if M.is_multiple_of(i) {
                        return false;
                    }
                    i += 2;
                }
                true
            }
        }
    }

    impl<const MOD: u32> ModInt<MOD> {
        const MOD_IS_PRIME: bool = check_primary::<MOD>();

        pub const ZERO: Self = unsafe { Self::from_raw(0) };
        pub const ONE: Self = Self::new(1);

        pub const fn new(x: i64) -> Self {
            unsafe { Self::from_raw(x.rem_euclid(MOD as i64) as u32) }
        }

        pub const fn from_usize(x: usize) -> Self {
            unsafe { Self::from_raw((x % MOD as usize) as u32) }
        }

        pub const fn get(self) -> u32 {
            self.0
        }

        /// # Safety
        ///
        /// * xがMOD未満であること
        pub const unsafe fn from_raw(x: u32) -> Self {
            const {
                assert!(MOD <= 1 << 31, "Maximum value of MOD is 2^31.");
                assert!(MOD > 0, "MOD must be non-zero.");
            }
            unsafe {
                std::hint::assert_unchecked(x < MOD);
            }
            Self(x)
        }

        pub const fn add_const(self, rhs: Self) -> Self {
            unsafe {
                let sum = self.0.unchecked_add(rhs.0);
                Self::from_raw(if sum >= MOD { sum - MOD } else { sum })
            }
        }

        pub const fn sub_const(self, rhs: Self) -> Self {
            unsafe {
                let diff = self.0.unchecked_add(MOD).unchecked_sub(rhs.0);
                Self::from_raw(if diff >= MOD { diff - MOD } else { diff })
            }
        }

        pub const fn mul_const(self, rhs: Self) -> Self {
            unsafe { Self::from_raw((self.0 as u64 * rhs.0 as u64 % MOD as u64) as u32) }
        }

        pub const fn neg_const(self) -> Self {
            unsafe { Self::from_raw(if self.0 == 0 { 0 } else { MOD - self.0 }) }
        }

        pub const fn pow(self, mut exp: u32) -> Self {
            if MOD == 1 {
                return Self::ZERO;
            }
            let mut result = Self::ONE;
            let mut base = self;
            while exp > 0 {
                if exp & 1 == 1 {
                    result = result.mul_const(base);
                }
                base = base.mul_const(base);
                exp >>= 1;
            }
            result
        }

        pub const fn inv(self) -> Self {
            const {
                if !Self::MOD_IS_PRIME {
                    panic!("Cannot calculate the inverse of a number in a non-prime modulo.");
                }
            }
            self.pow(MOD - 2)
        }

        pub const fn as_u32_slice(slice: &[Self]) -> &[u32] {
            unsafe { std::slice::from_raw_parts(slice.as_ptr() as *const _, slice.len()) }
        }
    }

    impl<const MOD: u32> Add for ModInt<MOD> {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            self.add_const(rhs)
        }
    }

    impl<const MOD: u32> AddAssign for ModInt<MOD> {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs;
        }
    }

    impl<const MOD: u32> Sub for ModInt<MOD> {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            self.sub_const(rhs)
        }
    }

    impl<const MOD: u32> SubAssign for ModInt<MOD> {
        fn sub_assign(&mut self, rhs: Self) {
            *self = *self - rhs;
        }
    }

    impl<const MOD: u32> Mul for ModInt<MOD> {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self::Output {
            self.mul_const(rhs)
        }
    }

    impl<const MOD: u32> Mul<usize> for ModInt<MOD> {
        type Output = Self;

        fn mul(self, rhs: usize) -> Self::Output {
            let rhs = Self::from_usize(rhs);
            self * rhs
        }
    }

    impl<const MOD: u32> MulAssign for ModInt<MOD> {
        fn mul_assign(&mut self, rhs: Self) {
            *self = *self * rhs;
        }
    }

    impl<const MOD: u32> MulAssign<usize> for ModInt<MOD> {
        fn mul_assign(&mut self, rhs: usize) {
            *self = *self * rhs;
        }
    }

    impl<const MOD: u32> Div for ModInt<MOD> {
        type Output = Self;

        fn div(self, rhs: Self) -> Self::Output {
            self.mul_const(rhs.inv())
        }
    }

    impl<const MOD: u32> DivAssign for ModInt<MOD> {
        fn div_assign(&mut self, rhs: Self) {
            *self = *self / rhs;
        }
    }

    impl<const MOD: u32> Neg for ModInt<MOD> {
        type Output = Self;
        fn neg(self) -> Self::Output {
            self.neg_const()
        }
    }

    impl<const MOD: u32> Display for ModInt<MOD> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    pub mod combination {
        use super::ModInt;

        pub struct Factorials<const MOD: u32> {
            fact: Box<[ModInt<MOD>]>,
            fact_inverse: Box<[ModInt<MOD>]>,
        }

        impl<const MOD: u32> Factorials<MOD> {
            pub fn new(max_n: usize) -> Self {
                let mut fact = Box::new_uninit_slice(max_n + 1);
                let mut fact_inverse = Box::new_uninit_slice(max_n + 1);
                let mut cur = ModInt::<MOD>::ONE;
                fact[0].write(cur);
                for (i, fi) in fact.iter_mut().enumerate().skip(1) {
                    cur *= i;
                    fi.write(cur);
                }
                cur = cur.inv();
                fact_inverse[max_n].write(cur);
                for (i, fi) in fact_inverse.iter_mut().enumerate().rev().skip(1) {
                    cur *= i + 1;
                    fi.write(cur);
                }
                unsafe {
                    Self {
                        fact: fact.assume_init(),
                        fact_inverse: fact_inverse.assume_init(),
                    }
                }
            }

            /// * calculate nPk
            pub fn permutation(&self, n: usize, k: usize) -> ModInt<MOD> {
                if k > n {
                    ModInt::ZERO
                } else {
                    self.fact[n] * self.fact_inverse[n - k]
                }
            }

            /// * calculate nCk
            pub fn combination(&self, n: usize, k: usize) -> ModInt<MOD> {
                if k > n {
                    ModInt::ZERO
                } else {
                    self.fact[n] * self.fact_inverse[k] * self.fact_inverse[n - k]
                }
            }

            /// * calculate nHk = (n + k - 1)Ck
            /// * 区別できるn個のものから重複を許してk個選ぶ方法の通り数
            pub fn multi_choose(&self, n: usize, k: usize) -> ModInt<MOD> {
                self.fact[n + k - 1] * self.fact_inverse[n - 1] * self.fact_inverse[k]
            }

            /// calculate n!
            pub fn fact(&self, n: usize) -> ModInt<MOD> {
                self.fact[n]
            }

            /// calculate 1 / n!
            pub fn fact_inv(&self, n: usize) -> ModInt<MOD> {
                self.fact_inverse[n]
            }
        }
    }
}
