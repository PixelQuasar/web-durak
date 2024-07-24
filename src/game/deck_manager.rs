use std::collections::HashMap;
use std::fmt;
use rand::rngs::StdRng;
use rand::{SeedableRng};
use rand::seq::SliceRandom;

#[derive(Clone, Copy, PartialEq)]
pub struct Card {
    pub suit: i32,
    pub rank: i32
}

impl Card {
    pub fn new(rank: i32, suit: i32) -> Card {
        Card { rank, suit }
    }
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}-{}] {} of {}",
               self.rank,
               self.suit,
               vec![
                   "unknown", "two", "three", "four", "five", "six", "seven", "eight",
                   "nine", "ten", "jack", "queen", "king", "ace"
               ].get(self.rank as usize).unwrap(),
               vec![
                   "unknown", "hearts", "diamonds", "spades", "clubs"
               ].get(self.suit as usize).unwrap())
    }
}

pub fn generate_deck(suit_num: i32, cards_num: i32) -> Vec<Card> {
    let mut cards = vec![];
    for i in 1..cards_num+1 {
        for j in 1..suit_num+1 {
            cards.push(Card{ suit: j, rank: i });
        }
    }
    cards
}

#[derive(Debug, Clone)]
pub struct DeckManager {
    full_deck: Vec<Card>,
    deck: Vec<Card>,
    discard: Vec<Card>,
    hands: HashMap<String, Vec<Card>>,
    hands_amount: usize,
    trump_suit: i32,
    table: Vec<(Card, Option<Card>)>
}

impl DeckManager {
    pub fn new() -> DeckManager {
        DeckManager {
            full_deck: generate_deck(4, 13),
            deck: generate_deck(4, 13),
            discard: vec![],
            hands: HashMap::new(),
            hands_amount: 0,
            trump_suit: 0,
            table: vec![]
        }
    }

    pub fn deal(&mut self, players_vec: Vec<String>, hand_size: i32) {
        let players_num = players_vec.len();
        self.hands_amount = players_num;
        self.deck.shuffle(&mut StdRng::seed_from_u64(1234));
        //self.deck.shuffle(&mut thread_rng());
        self.trump_suit = self.deck[self.deck.len() - 1].suit;
        for player_id in players_vec {
            let mut new_hand = vec![];
            for _ in 0..hand_size {
                new_hand.push(self.deck.pop().unwrap());
            }
            self.hands.insert(player_id, new_hand);
        }
    }

    pub fn deal_six(&mut self, players_vec: Vec<String>) {
        self.deal(players_vec, 6);
    }

    pub fn init_table(&mut self, player_id: &str, card: Card) -> Result<(), ()> {
        if !self.player_has_card(&player_id, card) {
            return Err(());
        }

        self.table.push((card, None));

        self.pick_card(&player_id, card)?;

        Ok(())
    }

    pub fn toss(&mut self, player_id: &str, card: Card) -> Result<(), ()> {
        if !self.player_has_card(&player_id, card) || !self.can_toss(card) {
            return Err(());
        }

        self.table.push((card, None));

        self.pick_card(&player_id, card)?;

        Ok(())
    }

    pub fn beat(&mut self, player_id: &str, beating: Card, beatable: Card) -> Result<(), ()> {
        if !self.player_has_card(&player_id, beating) ||
            !self.table_has_open_card(beatable) ||
            !self.can_beat(beating, beatable) {
            return Err(());
        }

        for i in 0..self.table.len() {
            let (bottom, _) = self.table[i];
            if bottom == beatable {
                self.table[i].1 = Some(beating)
            }
        }

        self.pick_card(&player_id, beating)?;

        Ok(())
    }

    pub fn take_table(&mut self, player_id: &str) -> Result<(), ()> {
        for pair in &self.table {
            let player = match self.hands.get_mut(player_id) {
                Some(player) => player,
                None => {
                    return Err(())
                }
            };
            player.push(pair.0);
            if pair.1.is_some() {
                player.push(pair.1.unwrap());
            }
        }
        self.table = vec![];

        Ok(())
    }

    pub fn discard_table(&mut self) {
        for pair in &self.table {
            self.discard.push(pair.0);
            if pair.1.is_some() {
                self.discard.push(pair.1.unwrap());
            }
        }
        self.table = vec![];
    }

    pub fn can_beat(&self, beating: Card, beatable: Card) -> bool {
        (beating.suit == beatable.suit && beating.rank > beatable.rank) ||
        (beating.suit == self.trump_suit && beatable.suit != self.trump_suit)
    }

    fn flatten_table(&self) -> Vec<Card> {
        let mut result = Vec::<Card>::new();
        for pair in &self.table {
            result.push(pair.0);
            if pair.1.is_some() {
                result.push(pair.1.unwrap());
            }
        }
        result
    }

    fn player_has_card(&self, player_id: &str, card: Card) -> bool {
        let hand = self.hands.get(player_id);

        if hand.is_none() {
            return false;
        }

        hand.unwrap().contains(&card)
    }

    fn pick_card(&mut self, player_id: &str, card: Card) -> Result<(), ()> {
        if !self.player_has_card(player_id, card) {
            return Err(())
        }
        let hand = self.hands.get_mut(player_id).unwrap();

        let card_index = hand.iter().position(|item| *item == card).unwrap();

        hand.remove(card_index);

        Ok(())
    }

    fn table_has_open_card(&self, card: Card) -> bool {
        for (bottom, top) in &self.table {
            if top.is_none() && *bottom == card {
                return true;
            }
        }
        false
    }

    fn can_toss(&self, card: Card) -> bool {
        for (bottom, top) in &self.table {
            if bottom.rank == card.rank {
                return true;
            }
            if let Some(top) = top {
                if top.rank == card.rank {
                    return true;
                }
            }
        }
        false
    }
}
