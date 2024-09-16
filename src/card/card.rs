use crate::mechanics::mana;

use image::DynamicImage;

use std::collections::HashSet;

pub struct Card{
    name: String,
    card_types: HashSet<String>,
    description: String,
    cost: mana::ManaCost,
    artwork: DynamicImage,
}

impl Card{

}
