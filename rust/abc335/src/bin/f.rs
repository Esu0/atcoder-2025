type MInt = modulo::ModInt<998244353>;
const B: usize = 450;

fn main() {
    proconio::input! {
        n: usize,
        a: [usize; n],
    }
    let mut sum = [[MInt::new(0); B]; B + 1];
    let mut dp = vec![MInt::new(1); n];

    for (i, &ai) in a.iter().enumerate().rev() {
        if ai > B {
            let mut j = i + ai;
            while j < n {
                dp[i] = dp[i] + dp[j];
                j += ai;
            }
        } else {
            dp[i] += sum[ai][i % ai];
        }
        for k in 1..=B {
            sum[k][i % k] += dp[i];
        }
    }
    println!("{}", dp[0]);
}
pub mod modulo {
    use std::{
        fmt::{self, Display},
        ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
    };

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Default, PartialOrd, Ord, Hash)]
    pub struct ModInt<const MOD: u32>(u32);
    const fn check_primary<const M: u32>() -> bool {
        match M {
            0 => false,
            1 => false,
            2 => true,
            _ => {
                if M % 2 == 0 {
                    return false;
                }
                let mut i = 3;
                while i * i <= M {
                    if M % i == 0 {
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

        pub const fn new(x: i64) -> Self {
            Self(x.rem_euclid(MOD as i64) as u32)
        }

        pub const fn get(self) -> u32 {
            self.0
        }

        pub const fn add_const(self, rhs: Self) -> Self {
            let sum = self.0 as u64 + rhs.0 as u64;
            if sum >= MOD as u64 {
                Self((sum - MOD as u64) as u32)
            } else {
                Self(sum as u32)
            }
        }

        pub const fn sub_const(self, rhs: Self) -> Self {
            let diff = self.0 as u64 + MOD as u64 - rhs.0 as u64;
            if diff >= MOD as u64 {
                Self((diff - MOD as u64) as u32)
            } else {
                Self(diff as u32)
            }
        }

        pub const fn mul_const(self, rhs: Self) -> Self {
            Self((self.0 as u64 * rhs.0 as u64 % MOD as u64) as u32)
        }

        pub const fn pow(self, mut exp: u32) -> Self {
            let mut result = Self(1);
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
            if !Self::MOD_IS_PRIME {
                panic!("Cannot calculate the inverse of a number in a non-prime modulo.");
            }
            self.pow(MOD - 2)
        }
    }

    impl<const MOD: u32> Add for ModInt<MOD> {
        type Output = Self;

        fn add(self, rhs: Self) -> Self::Output {
            self.add_const(rhs)
        }
    }

    impl<const MOD: u32> Add<u32> for ModInt<MOD> {
        type Output = Self;

        fn add(self, rhs: u32) -> Self::Output {
            self.add_const(Self(rhs % MOD))
        }
    }

    impl<const MOD: u32> Add<u64> for ModInt<MOD> {
        type Output = Self;

        fn add(self, rhs: u64) -> Self::Output {
            self.add_const(Self((rhs % MOD as u64) as u32))
        }
    }

    impl<const MOD: u32> AddAssign for ModInt<MOD> {
        fn add_assign(&mut self, rhs: Self) {
            *self = *self + rhs;
        }
    }

    impl<const MOD: u32> AddAssign<u32> for ModInt<MOD> {
        fn add_assign(&mut self, rhs: u32) {
            *self = *self + rhs;
        }
    }

    impl<const MOD: u32> AddAssign<u64> for ModInt<MOD> {
        fn add_assign(&mut self, rhs: u64) {
            *self = *self + rhs;
        }
    }

    impl<const MOD: u32> Sub for ModInt<MOD> {
        type Output = Self;

        fn sub(self, rhs: Self) -> Self::Output {
            self.sub_const(rhs)
        }
    }

    impl<const MOD: u32> Sub<u32> for ModInt<MOD> {
        type Output = Self;

        fn sub(self, rhs: u32) -> Self::Output {
            self.sub_const(Self(rhs % MOD))
        }
    }

    impl<const MOD: u32> Sub<u64> for ModInt<MOD> {
        type Output = Self;

        fn sub(self, rhs: u64) -> Self::Output {
            self.sub_const(Self((rhs % MOD as u64) as u32))
        }
    }

    impl<const MOD: u32> SubAssign for ModInt<MOD> {
        fn sub_assign(&mut self, rhs: Self) {
            *self = *self - rhs;
        }
    }

    impl<const MOD: u32> SubAssign<u32> for ModInt<MOD> {
        fn sub_assign(&mut self, rhs: u32) {
            *self = *self - rhs;
        }
    }

    impl<const MOD: u32> SubAssign<u64> for ModInt<MOD> {
        fn sub_assign(&mut self, rhs: u64) {
            *self = *self - rhs;
        }
    }

    impl<const MOD: u32> Mul for ModInt<MOD> {
        type Output = Self;

        fn mul(self, rhs: Self) -> Self::Output {
            self.mul_const(rhs)
        }
    }

    impl<const MOD: u32> Mul<u32> for ModInt<MOD> {
        type Output = Self;

        fn mul(self, rhs: u32) -> Self::Output {
            Self((self.0 as u64 * rhs as u64 % MOD as u64) as u32)
        }
    }

    impl<const MOD: u32> Mul<u64> for ModInt<MOD> {
        type Output = Self;

        fn mul(self, rhs: u64) -> Self::Output {
            Self((self.0 as u64 * (rhs % MOD as u64) % MOD as u64) as u32)
        }
    }

    impl<const MOD: u32> MulAssign for ModInt<MOD> {
        fn mul_assign(&mut self, rhs: Self) {
            *self = *self * rhs;
        }
    }

    impl<const MOD: u32> MulAssign<u32> for ModInt<MOD> {
        fn mul_assign(&mut self, rhs: u32) {
            *self = *self * rhs;
        }
    }

    impl<const MOD: u32> MulAssign<u64> for ModInt<MOD> {
        fn mul_assign(&mut self, rhs: u64) {
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

    impl<const MOD: u32> Display for ModInt<MOD> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }
}
