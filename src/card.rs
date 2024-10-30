use rand::seq::SliceRandom;

pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Deck {
        let mut cards = Vec::new();
        for color in [Color::Spades, Color::Hearts, Color::Diamonds, Color::Clubs].iter() {
            for i in 7..11 {
                let card = Card {
                    color: *color,
                    default_value: i,
                    trump_value: if i == 9 { 14 } else { i },
                    is_trump: false,
                    name: i.to_string(),
                };
                cards.push(card);
            }
            let ace = Card {
                color: *color,
                default_value: 11,
                name: "A".to_string(),
                ..Default::default()
            };
            cards.push(ace);
            let queen = Card {
                color: *color,
                default_value: 3,
                name: "Q".to_string(),
                ..Default::default()
            };
            cards.push(queen);
            let king = Card {
                color: *color,
                default_value: 4,
                name: "K".to_string(),
                ..Default::default()
            };
            cards.push(king);
            let jack = Card {
                color: *color,
                default_value: 2,
                trump_value: 20,
                name: "J".to_string(),
                ..Default::default()
            };
            cards.push(jack);
        }
        Deck { cards }
    }

    pub fn shuffle(&mut self) -> &Deck {
        let mut rng = rand::thread_rng();
        self.cards.shuffle(&mut rng);
        return self;
    }

    pub fn draw(&mut self) -> Card {
        self.cards.pop()
            .unwrap_or_else(|| {
                println!("No more cards in the deck");
                Default::default()
            })
    }

}

#[derive(Clone, Copy, Debug)]
pub enum Color {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
}

#[derive(Clone, Debug)]
pub struct Card {
    color: Color,
    default_value: i8,
    trump_value: i8,
    is_trump: bool,
    name: String,
}

impl Default for Card {
    fn default() -> Self {
        Card {
            color: Color::Spades,
            default_value: 0,
            trump_value: 0,
            is_trump: false,
            name: String::from(""),
        }
    }
}

impl Card {
    pub fn display(&self) {
        let color = match self.color {
            Color::Spades => "♠",
            Color::Hearts => "♥",
            Color::Diamonds => "♦",
            Color::Clubs => "♣",
        };
        println!("{} {}", color, self.name);
    }

    pub fn get_value(&self) -> i8 {
        if self.is_trump {
            self.trump_value
        } else {
            self.default_value
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_color(self) -> Color {
        self.color
    }
}
