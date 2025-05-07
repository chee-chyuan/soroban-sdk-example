use soroban_sdk::{contracttype, Env, Vec};

use super::elem::Elem;

const EXT_SIZE: usize = 4;

#[contracttype]
#[derive(Clone)]
pub struct ExtElem(pub Vec<Elem>);

pub type BabyBearExtElem = ExtElem;

impl ExtElem {
    const INVALID: [Elem; 4] = [Elem::INVALID, Elem::INVALID, Elem::INVALID, Elem::INVALID];

    pub fn new(env: &Env, x0: Elem, x1: Elem, x2: Elem, x3: Elem) -> Self {
        Self(Vec::from_array(env, [x0, x1, x2, x3]))
    }

    pub fn from_fp(env: &Env, x: Elem) -> Self {
        Self(Vec::from_array(
            env,
            [x, Elem::new(0), Elem::new(0), Elem::new(0)],
        ))
    }

    /// Create an [ExtElem] from a raw integer.
    pub fn from_u32(env: &Env, x0: u32) -> Self {
        Self(Vec::from_array(
            env,
            [Elem::new(x0), Elem::new(0), Elem::new(0), Elem::new(0)],
        ))
    }

    /// Return the value zero.
    fn zero(env: &Env) -> Self {
        Self::from_u32(env, 0)
    }

    /// Return the value one.
    fn one(env: &Env) -> Self {
        Self::from_u32(env, 1)
    }

    fn is_valid(&self) -> bool {
        self.0
            .iter()
            .zip(Self::INVALID.iter())
            .all(|(a, b)| a == *b)
    }

    fn ensure_valid(&self) -> Self {
        if !self.is_valid() {
            panic!("Invalid Baby Bear element");
        }
        self.clone()
    }

    pub fn const_part(self) -> Elem {
        self.ensure_valid().0.get(0).unwrap()
    }

    /// Return [Elem] as a vector of base field values.
    pub fn elems(&self) -> Vec<Elem> {
        self.ensure_valid().0
    }
}
