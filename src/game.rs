use crate::card::{Card, Color, Deck};
use crate::player::{Player, Team};

pub struct Game<'a> {
    pub players: Vec<Player>,
    pub deck: Deck,
    pub trump_color: Option<Color>,
    pub taker: Option<&'a Player>,
    pub current_round: i8,
    pub current_player: Option<&'a Player>,
    pub maximum_points: i32,
}

const FIRST_DRAW: i8 = 3;
const SECOND_DRAW: i8 = 2;

impl Game<'a> {
    pub fn new(players: Vec<Player>) -> Game<'a> {
        Game {
            players,
            ..Default::default()
        }
    }

    fn set_teams(&mut self) -> (Team, Team) {
        let red = Team::new(vec![self.players[0].clone(), self.players[2].clone()]);
        let black = Team::new(vec![self.players[1].clone(), self.players[3].clone()]);
        (red, black)
    }

    pub fn start(&mut self) {
        self.deck.shuffle();
        let (red, black) = self.set_teams();

        while (red.points < self.maximum_points) && (black.points < self.maximum_points) {
            self.start_round();
        }
    }

    fn start_round(&mut self) {
        self.current_round += 1;
        self.draw_starting_hands();
        let potential_trump = &self.deck.draw();
        println!("Potential trump: {:?}", potential_trump);
        let trump_taken_first = self.wait_for_trump_first(potential_trump);
        self.current_player = Some(&self.players[0]);
        if trump_taken_first {
            self.draw_remaining_cards();
            self.play_round();
        } else {
            let trump_taken_second = self.wait_for_trump_second(potential_trump);
            if trump_taken_second {
                self.play_round();
            } else {
                println!("No one took the trump! New round starting...");
            }
        }
    }

    fn draw_starting_hands(&mut self) {
        for player in &mut self.players {
            player.draw_n(&mut self.deck, FIRST_DRAW);
        }
        for player in &mut self.players {
            player.draw_n(&mut self.deck, SECOND_DRAW);
        }
    }

    fn draw_remaining_cards(&mut self) {
        println!("{} took the trump", self.taker.name);
        for player in &mut self.players {
            if player.name == self.taker.name {
                player.draw_n(&mut self.deck, 2);
            } else {
                player.draw_n(&mut self.deck, 3);
            }
            player.display_hand();
        }
    }

    fn wait_for_trump_first(&mut self, potential_trump: &Card) -> bool {
        for player in &mut self.players {
            let (does_want, _) = player.consider_trump(&potential_trump);
            if does_want {
                self.trump_color = Some(potential_trump.get_color());
                self.taker = Some(player);
                player.hand.push(potential_trump.clone());
                break;
            }
        }
        return self.taker.is_some();
    }

    fn wait_for_trump_second(&mut self, potential_trump: &Card) -> bool {
        let mut taker_index: Option<usize> = None;
        for (index, player) in self.players.iter_mut().enumerate() {
            let (does_want, color) = player.consider_trump(&potential_trump);
            if does_want {
                self.trump_color = Some(color.unwrap());
                self.taker = Some(player);
                player.hand.push(potential_trump.clone());
                break;
            }
        }
        return self.taker.is_some();
    }

    fn play_round(&mut self) {
        // TODO: Implement
    }
}

impl Default for Game<'_> {
    fn default() -> Self {
        Game {
            players: Vec::new(),
            deck: Deck::new(),
            trump_color: None,
            taker: None,
            current_round: 0,
            current_player: None,
            maximum_points: 1000,
        }
    }
}
