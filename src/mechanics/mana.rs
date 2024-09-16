use std::collections::HashMap;

pub enum ManaColor{
    White,
    Blue,
    Black,
    Red,
    Green,
    Colorless,
}

pub enum Mana{
    MonoColor(ManaColor),
    HybridColor(ManaColor, ManaColor),
    PhyrexianMonoColor(ManaColor),
    PhyrexianHybridColor(ManaColor, ManaColor),
}

pub struct ManaCost{
    cost: HashMap<Mana, u8>,
}

impl ManaCost{
    pub fn new() -> Self{
        todo!();
    }
}
