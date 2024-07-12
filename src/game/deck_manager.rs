use std::fmt;
use rand::thread_rng;
use rand::seq::SliceRandom;

#[derive(Clone, Copy)]
pub struct Card {
    suit: i32,
    rank: i32
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} of {}",
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
            cards.push(Card{
                suit: j,
                rank: i
            });
        }
    }
    cards
}

#[derive(Debug, Clone)]
pub struct DeckManager {
    full_deck: Vec<Card>,
    deck: Vec<Card>,
    discard: Vec<Card>,
    hands: Vec<Vec<Card>>,
    trump_suit: i32,
}

impl DeckManager {
    pub fn new() -> DeckManager {
        DeckManager {
            full_deck: generate_deck(4, 13),
            deck: generate_deck(4, 13),
            discard: vec![],
            hands: vec![],
            trump_suit: 0
        }
    }

    pub fn deal(&mut self, players_num: i32, hand_size: i32) {
        self.deck.shuffle(&mut thread_rng());
        self.trump_suit = self.deck[self.deck.len() - 1].suit;
        for _ in 0..players_num {
            let mut new_hand = vec![];
            for _ in 0..hand_size {
                new_hand.push(self.deck.pop().unwrap());
            }
            self.hands.push(new_hand);
        }
    }

    pub fn deal_six(&mut self, players_num: i32) {
        self.deal(players_num, 6);
    }
}
