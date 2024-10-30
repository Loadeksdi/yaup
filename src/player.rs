use crate::card::{Card, Color, Deck};

pub struct Team {
    pub players: Vec<Player>,
    pub points: i32,
}

impl Default for Team {
    fn default() -> Team {
        Team {
            players: Vec::new(),
            points: 0,
        }
    }
}

impl Team {
    pub fn new(players: Vec<Player>) -> Team {
        Team {
            players,
            ..Default::default()
        }
    }
}

#[derive(Clone, Debug)]
pub struct Player {
    pub name: String,
    pub hand: Vec<Card>,
}

impl Player {
    pub fn new(name: &str) -> Player {
        Player {
            name: String::from(name),
            hand: Vec::new(),
        }
    }

    pub fn draw_n(&mut self, deck: &mut Deck, x: i8) {
        for _ in 0..x {
            self.hand.push(deck.draw());
        }
    }

    pub fn display_hand(&self) {
        println!("{}'s hand:", self.name);
        for card in &self.hand {
            card.display();
        }
    }

    pub fn consider_trump(&mut self, card: &Card) -> (bool, Option<Color>) {
        card.display();
        return (&self.name == "Progress", Some(Color::Spades));
    }
}
