use crate::card::{Card, Color, Deck};
use crate::player::{Player, Team};

pub struct Round<'a> {
    pub current: i8,
    pub trump_color: Option<Color>,
    pub current_player: Option<&'a Player>,
    pub taker: Option<&'a Player>,
    pub trick: Vec<&Card>,
    pub players: Vec<&Player>,
    pub deck: Deck
}

impl Round {

    pub fn new(count: i8, players: Vec<&Player>, deck: Deck) -> Self {
        return Round {
            current: count,
            trump_color: None,
            current_player: None,
            taker: None,
            trick: Vec::new(),
            players: players,
            deck: deck
        }
    }

    pub fn start(&mut self) {
        self.current += 1;
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
        println!("{} took the trump", &self.taker);
        for player in &mut self.players {
            if player.name == &self.taker.unwrap().name {
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
                player.hand.push(potential_trump);
                break;
            }
        }
        return self.taker.is_some();
    }

    pub fn play_round() {
        // TODO implement
    }
}

pub struct Game {
    pub teams: (Team, Team),
    pub deck: Deck,
    pub rounds: Vec<Round>,
    pub maximum_points: i8,
}

const FIRST_DRAW: i8 = 3;
const SECOND_DRAW: i8 = 2;
const MAXIMUM_POINTS: i8 = 1000;

impl Game {
    pub fn new(teams: (Team, Team)) -> Game {
        Game {
            deck: Deck::new(),
            rounds: Vec::new(),
            maximum_points: MAXIMUM_POINTS,
            teams: teams
        }
    }

    pub fn start(&mut self) {
        self.deck.shuffle();
        let mut counter = 0;
        while (self.teams.0.points < self.maximum_points) && (self.teams.1.points < self.maximum_points) {
            self.start_round(counter);
            counter += 1;
        }
    }

    fn start_round(&mut self, count: i8) {
        let round = Round::new(count, self.get_players(), Deck::new());        
    }

    fn get_players(&mut self) -> Vec<&Player> {
        vec![&self.teams.0.players[0], &self.teams.1.players[0], &self.teams.0.players[1], &self.teams.1.players[1]]
    }

}
