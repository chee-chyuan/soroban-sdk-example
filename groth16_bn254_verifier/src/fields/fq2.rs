use soroban_sdk::contracttype;

#[contracttype]
#[derive(Clone)]
pub struct Fq2 {
    pub x: u128, // temporary because unit structures are not supported
}
