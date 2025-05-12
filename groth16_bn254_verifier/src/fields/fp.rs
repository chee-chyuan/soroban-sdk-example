use soroban_sdk::{contracttype, Env, Vec};

#[contracttype]
#[derive(Clone)]
pub struct Fp {
    pub bigint: Vec<u64>,
}

impl Fp {
    pub const ZERO: [u64; 4] = [0u64; 4];
    pub const N: usize = 4;

    pub fn zero(env: &Env) -> Self {
        Fp {
            bigint: Vec::from_array(env, Self::ZERO),
        }
    }
}

// const ZERO: Fp<Self, N> = Fp::new_unchecked(BigInt([0u64; N]));

// /// Multiplicative identity of the field, i.e. the element `e`
// /// such that, for all elements `f` of the field, `e * f = f`.
// const ONE: Fp<Self, N> = Fp::new_unchecked(T::R);

// const fn const_mul2_with_carry(mut self) -> (Self, bool) {
//     let mut last = 0;
//     crate::const_for!((i in 0..N) {
//         let a = self.0[i];
//         let tmp = a >> 63;
//         self.0[i] <<= 1;
//         self.0[i] |= last;
//         last = tmp;
//     });
//     (self, last != 0)
// }

// const MODULUS: BigInt<N>;

//     /// Let `M` be the power of 2^64 nearest to `Self::MODULUS_BITS`. Then
//     /// `R = M % Self::MODULUS`.
//     const R: BigInt<N> = Self::MODULUS.montgomery_r();



pub struct RBuffer(pub [u64; Fp::N], pub u64);

impl RBuffer {
    pub const fn num_bits(&self) -> u32 {
        (Fp::N * 64) as u32 + (64 - self.1.leading_zeros())
    }

    pub const fn get_bit(&self, i: usize) -> bool {
        let d = i / 64;
        let b = i % 64;
        if d == Fp::N {
            (self.1 >> b) & 1 == 1
        } else {
            (self.0[d] >> b) & 1 == 1
        }
    }
}

pub struct BigInt(pub [u64; Fp::N]);

#[macro_export]
macro_rules! const_for {
    (($i:ident in $start:tt..$end:tt)  $code:expr ) => {{
        let mut $i = $start;
        while $i < $end {
            $code
            $i += 1;
        }
    }};
}

#[macro_export]
macro_rules! sbb {
    ($a:expr, $b:expr, &mut $borrow:expr$(,)?) => {{
        let tmp = (1u128 << 64) + ($a as u128) - ($b as u128) - ($borrow as u128);
        $borrow = if tmp >> 64 == 0 { 1 } else { 0 };
        tmp as u64
    }};
}

impl BigInt {
    const fn const_mul2_with_carry(mut self) -> (Self, bool) {
        let mut last = 0;
        const N: usize = Fp::N;
        crate::const_for!((i in 0..N) {
            let a = self.0[i];
            let tmp = a >> 63;
            self.0[i] <<= 1;
            self.0[i] |= last;
            last = tmp;
        });
        (self, last != 0)
    }

    const fn const_geq(&self, other: &Self) -> bool {
        const N: usize = Fp::N;
        const_for!((i in 0..N) {
            let a = self.0[N - i - 1];
            let b = other.0[N - i - 1];
            if a < b {
                return false;
            } else if a > b {
                return true;
            }
        });
        true
    }
    pub const fn const_sub_with_borrow(mut self, other: &Self) -> (Self, bool) {
        let mut borrow = 0;
        const N: usize = Fp::N;
        const_for!((i in 0..N) {
            self.0[i] = sbb!(self.0[i], other.0[i], &mut borrow);
        });

        (self, borrow != 0)
    }

    pub const fn const_modulo(a: RBuffer, divisor: &Self) -> Self {
        // assert!(!$divisor.const_is_zero());
        let mut remainder = Self([0u64; Fp::N]);
        let mut i = (a.num_bits() - 1) as isize;
        let mut carry;
        while i >= 0 {
            (remainder, carry) = remainder.const_mul2_with_carry();
            remainder.0[0] |= a.get_bit(i as usize) as u64;
            if remainder.const_geq(divisor) || carry {
                let (r, borrow) = remainder.const_sub_with_borrow(divisor);
                remainder = r;
                assert!(borrow == carry);
            }
            i -= 1;
        }
        remainder
    }

    pub const fn montgomery_r(&self) -> Self {
        let two_pow_n_times_64 = RBuffer([0u64; Fp::N], 1);
        Self::const_modulo(two_pow_n_times_64, self)
    }
}

// pub type Fq = Fp256<MontBackend<FqConfig, 4>>;
// pub type Fp256<P> = Fp<P, 4>;

// #[derive(Copy, Clone, PartialEq, Eq, Hash, Zeroize)]
// pub struct BigInt<const N: usize>(pub [u64; N]);

// impl<P: FpConfig<N>, const N: usize> Zero for Fp<P, N> {
//     #[inline]
//     fn zero() -> Self {
//         P::ZERO
//     }

//     #[inline]
//     fn is_zero(&self) -> bool {
//         *self == P::ZERO
//     }
// }

// impl<P: FpConfig<N>, const N: usize> One for Fp<P, N> {
//     #[inline]
//     fn one() -> Self {
//         P::ONE
//     }

//     #[inline]
//     fn is_one(&self) -> bool {
//         *self == P::ONE
//     }
// }
