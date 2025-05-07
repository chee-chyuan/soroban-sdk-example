use core::{cmp::Ordering, ops};

use bytemuck::Zeroable;
use soroban_sdk::contracttype;

/// The modulus of the field.
pub const P: u32 = 15 * (1 << 27) + 1;

/// The modulus of the field as a u64.
const P_U64: u64 = P as u64;

// montgomery form constants
const M: u32 = 0x88000001;
const R2: u32 = 1172168163;

#[contracttype]
#[derive(Eq, Clone, Copy, Zeroable)]
pub struct Elem(pub u32);

impl Elem {
    pub const INVALID: Self = Elem(0xffffffff);

    /// Create a new [BabyBear] from a raw integer.
    pub const fn new(x: u32) -> Self {
        Self(encode(x % P))
    }

    /// Create a new [BabyBear] from a Montgomery form representation
    ///
    /// Requires that `x` comes pre-encoded in Montgomery form.
    pub const fn new_raw(x: u32) -> Self {
        Self(x)
    }

    // /// Cast a [BabyBear] to an integer
    // pub const fn as_u32(&self) -> u32 {
    //     decode(self.0)
    // }

    /// Return the Montgomery form representation used for byte-based
    /// hashes of slices of [BabyBear]s.
    pub const fn as_u32_montgomery(&self) -> u32 {
        self.0
    }

    fn is_valid(&self) -> bool {
        self.0 != Self::INVALID.0
    }

    fn ensure_valid(&self) -> Self {
        if !self.is_valid() {
            panic!("Invalid Baby Bear element");
        }
        *self
    }

    pub fn is_reduced(&self) -> bool {
        self.0 < P
    }
}

impl ops::Add for Elem {
    type Output = Self;

    /// Addition for Baby Bear [Elem]
    fn add(self, rhs: Self) -> Self {
        Elem(add(self.ensure_valid().0, rhs.ensure_valid().0))
    }
}

impl ops::AddAssign for Elem {
    /// Simple addition case for Baby Bear [Elem]
    fn add_assign(&mut self, rhs: Self) {
        self.0 = add(self.ensure_valid().0, rhs.ensure_valid().0)
    }
}

impl ops::Sub for Elem {
    type Output = Self;

    /// Subtraction for Baby Bear [Elem]
    fn sub(self, rhs: Self) -> Self {
        Elem(sub(self.ensure_valid().0, rhs.ensure_valid().0))
    }
}

impl ops::SubAssign for Elem {
    /// Simple subtraction case for Baby Bear [Elem]
    fn sub_assign(&mut self, rhs: Self) {
        self.0 = sub(self.ensure_valid().0, rhs.ensure_valid().0)
    }
}

impl ops::Mul for Elem {
    type Output = Self;

    /// Multiplication for Baby Bear [Elem]
    fn mul(self, rhs: Self) -> Self {
        Elem(mul(self.ensure_valid().0, rhs.ensure_valid().0))
    }
}

impl ops::MulAssign for Elem {
    /// Simple multiplication case for Baby Bear [Elem]
    fn mul_assign(&mut self, rhs: Self) {
        self.0 = mul(self.ensure_valid().0, rhs.ensure_valid().0)
    }
}

impl ops::Neg for Elem {
    type Output = Self;

    fn neg(self) -> Self {
        Elem(0) - self.ensure_valid()
    }
}

impl PartialEq<Elem> for Elem {
    fn eq(&self, rhs: &Self) -> bool {
        self.ensure_valid().0 == rhs.ensure_valid().0
    }
}

impl Ord for Elem {
    fn cmp(&self, rhs: &Self) -> Ordering {
        decode(self.ensure_valid().0).cmp(&decode(rhs.ensure_valid().0))
    }
}

impl PartialOrd for Elem {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl From<Elem> for u32 {
    fn from(x: Elem) -> Self {
        decode(x.0)
    }
}

impl From<Elem> for u64 {
    fn from(x: Elem) -> Self {
        decode(x.0).into()
    }
}

impl From<u32> for Elem {
    fn from(x: u32) -> Self {
        Elem::new(x)
    }
}

impl From<u64> for Elem {
    fn from(x: u64) -> Self {
        Elem::new((x % P_U64) as u32)
    }
}

/// Wrapping addition of [Elem] using Baby Bear field modulus
fn add(lhs: u32, rhs: u32) -> u32 {
    let x = lhs.wrapping_add(rhs);
    if x >= P {
        x - P
    } else {
        x
    }
}

/// Wrapping subtraction of [Elem] using Baby Bear field modulus
fn sub(lhs: u32, rhs: u32) -> u32 {
    let x = lhs.wrapping_sub(rhs);
    if x > P {
        x.wrapping_add(P)
    } else {
        x
    }
}

/// Wrapping multiplication of [Elem]  using Baby Bear field modulus
// Copied from the C++ implementation (fp.h)
const fn mul(lhs: u32, rhs: u32) -> u32 {
    // uint64_t o64 = uint64_t(a) * uint64_t(b);
    let mut o64: u64 = (lhs as u64).wrapping_mul(rhs as u64);
    // uint32_t low = -uint32_t(o64);
    let low: u32 = 0u32.wrapping_sub(o64 as u32);
    // uint32_t red = M * low;
    let red = M.wrapping_mul(low);
    // o64 += uint64_t(red) * uint64_t(P);
    o64 += (red as u64).wrapping_mul(P_U64);
    // uint32_t ret = o64 >> 32;
    let ret = (o64 >> 32) as u32;
    // return (ret >= P ? ret - P : ret);
    if ret >= P {
        ret - P
    } else {
        ret
    }
}

/// Encode to Montgomery form from direct form.
const fn encode(a: u32) -> u32 {
    mul(R2, a)
}

/// Decode from Montgomery form to direct form.
const fn decode(a: u32) -> u32 {
    mul(1, a)
}
