use bytemuck::Zeroable;
use soroban_sdk::contracttype;

use super::elem::Elem;

const EXT_SIZE: usize = 4;

#[contracttype]
#[derive(Eq, Clone, Copy, Zeroable)]
pub struct ExtElem([Elem; EXT_SIZE]);

pub type BabyBearExtElem = ExtElem;

const fn const_ensure_valid(x: Elem) -> Elem {
    // debug_assert!(x.0 != Elem::INVALID.0);
    if x.0 == Elem::INVALID.0 {
        panic!("Invalid Baby Bear element");
    }
    x
}

impl ExtElem {
    const INVALID: Self = ExtElem([Elem::INVALID, Elem::INVALID, Elem::INVALID, Elem::INVALID]);
    /// Explicitly construct an ExtElem from parts.
    pub const fn new(x0: Elem, x1: Elem, x2: Elem, x3: Elem) -> Self {
        Self([
            const_ensure_valid(x0),
            const_ensure_valid(x1),
            const_ensure_valid(x2),
            const_ensure_valid(x3),
        ])
    }

    /// Create an [ExtElem] from an [Elem].
    pub fn from_fp(x: Elem) -> Self {
        Self([x, Elem::new(0), Elem::new(0), Elem::new(0)])
    }

    /// Create an [ExtElem] from a raw integer.
    pub const fn from_u32(x0: u32) -> Self {
        Self([Elem::new(x0), Elem::new(0), Elem::new(0), Elem::new(0)])
    }

    /// Return the value zero.
    const fn zero() -> Self {
        Self::from_u32(0)
    }

    /// Return the value one.
    const fn one() -> Self {
        Self::from_u32(1)
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

    /// Return the base field term of an [Elem].
    pub fn const_part(self) -> Elem {
        self.ensure_valid().0[0]
    }

    /// Return [Elem] as a vector of base field values.
    pub fn elems(&self) -> [Elem; EXT_SIZE] {
        self.ensure_valid().0
    }
}

impl PartialEq<ExtElem> for ExtElem {
    fn eq(&self, rhs: &Self) -> bool {
        self.ensure_valid().0 == rhs.ensure_valid().0
    }
}
