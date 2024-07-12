use rand::thread_rng;

#[derive(Debug, Clone, Copy)]
pub struct Card {
    suit: i32,
    rank: i32
}

pub fn generate_cards_list(suit_num: i32, cards_num: i32) -> Vec<Card> {
    let mut cards = vec![];
    for i in 1..cards_num+1 {
        for j in 1..suit_num+1 {
            cards.push(Card{
                suit: i,
                rank: j
            });
        }
    }
    cards
}

#[derive(Debug, Clone)]
pub struct Deck {
    full_cards: Vec<Card>,
    cards: Vec<Card>,
    discard: Vec<Card>,
    hands: Vec<Vec<Card>>,
    trump_suit: i32,
}

impl Deck {
    pub fn new() -> Deck {
        Deck {
            full_cards: generate_cards_list(4, 13),
            cards: generate_cards_list(4, 13),
            discard: vec![],
            hands: vec![],
            trump_suit: 0
        }
    }

    pub fn deal(&mut self, players_num: i32, hand_size: i32) {
        thread_rng().shuffle(&self.cards);
        for i in 0..hand_size {
            let mut new_hand = vec![];
            for i in 0..hand_size {
                new_hand.push(self.cards.pop().unwrap());
            }
            self.hands.push(new_hand);
        }
        self.trump_suit = self.cards.pop().unwrap().suit;
    }

    pub fn deal_six(&mut self, players_num: i32) {
        self.deal(players_num, 6);
    }
}
