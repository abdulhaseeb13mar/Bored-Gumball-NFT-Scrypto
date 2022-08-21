use sbor::*;

#[derive(TypeId, Encode, Decode, Describe)]
pub enum Color {
    Blue,
    Yellow,
    Red,
}

#[derive(TypeId, Encode, Decode, Describe)]
pub enum Hat {
    Beanie,
    Cowboy,
    Party,
}

#[derive(TypeId, Encode, Decode, Describe)]
pub enum Eyes {
    Laser,
    Sleepy,
    Eyepatch,
}
