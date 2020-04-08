use amethyst::ecs::{Component, DenseVecStorage, World, WorldExt};
use amethyst::prelude::Builder;

pub enum Suit {
    Green,
    Yellow,
    Red,
    Blue,
    Wizard,
    Jester,
}

pub enum Card {
    One { value: u8, suit: Suit },
    Two { value: u8, suit: Suit },
    Three { value: u8, suit: Suit },
    Four { value: u8, suit: Suit },
    Five { value: u8, suit: Suit },
    Six { value: u8, suit: Suit },
    Seven { value: u8, suit: Suit },
    Eight { value: u8, suit: Suit },
    Nine { value: u8, suit: Suit },
    Ten { value: u8, suit: Suit },
    Eleven { value: u8, suit: Suit },
    Twelve { value: u8, suit: Suit },
    Thirteen { value: u8, suit: Suit },
    Wizard { value: u8, suit: Suit },
    Jester { value: u8, suit: Suit },
}

pub struct Deck {
    cards: Vec<Card>,
}

impl Component for Deck {
    type Storage = DenseVecStorage<Self>;
}

pub fn initialize_deck(mut world: World) {
    // init all the struct values
    let mut entity_builder = world.create_entity();
    for i in 0..12 {

    }

}
