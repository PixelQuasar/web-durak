use std::collections::HashMap;
use std::fmt;
use rand::rngs::StdRng;
use rand::{SeedableRng};
use rand::seq::SliceRandom;
use serde::{Deserialize, Serialize, Serializer};
use crate::utils::generate_card_id;

pub type CardIdType = usize;

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Card {
    pub s: i32,
    pub r: i32,
    pub id: CardIdType
}

impl Card {
    pub fn new(rank: i32, suit: i32, id: CardIdType) -> Card {
        Card { r: rank, s: suit, id }
    }
}

impl fmt::Debug for Card {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}-{}] {} of {}",
               self.r,
               self.s,
               vec![
                   "unknown", "two", "three", "four", "five", "six", "seven", "eight",
                   "nine", "ten", "jack", "queen", "king", "ace"
               ].get(self.r as usize).unwrap(),
               vec![
                   "unknown", "hearts", "diamonds", "spades", "clubs"
               ].get(self.s as usize).unwrap())
    }
}

fn generate_deck(suit_num: i32, cards_num: i32) -> Vec<Card> {
    let mut cards = vec![];
    let mut id = 1;
    for i in 1..=cards_num {
        for j in 1..=suit_num {
            cards.push(Card::new(i, j, id));
            id += 1;
        }
    }
    cards
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeckManager {
    full_deck: Vec<Card>,
    deck: Vec<Card>,
    discard: Vec<Card>,
    hands: HashMap<String, Vec<Card>>,
    hands_amount: usize,
    hands_order: Vec<String>,
    beat_confirmations: HashMap<String, bool>,
    hand_size: usize,
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
            hands_order: vec![],
            hands_amount: 0,
            beat_confirmations: HashMap::new(),
            hand_size: 0,
            trump_suit: 0,
            table: vec![]
        }
    }

    pub fn deal(&mut self, players_vec: Vec<String>, hand_size: usize) {
        let players_num = players_vec.len();
        self.hands_amount = players_num;

        self.deck.shuffle(&mut StdRng::seed_from_u64(1234));

        self.trump_suit = self.deck[self.deck.len() - 1].s;

        self.hand_size = hand_size;

        for player_id in players_vec {
            self.hands.insert(player_id.clone(), vec![]);
            self.beat_confirmations.insert(player_id.clone(), false);
            self.deal_to_hand_until_full(&player_id).unwrap();
        }

        self.init_order()
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

    pub fn toss(&mut self, player_id: &str, card: Card) -> Result<i32, ()> {
        if !self.player_has_card(&player_id, card) || !self.can_toss(card) {
            return Err(());
        }

        self.table.push((card, None));

        self.pick_card(&player_id, card)?;

        Ok(self.table.len() as i32)
    }

    pub fn beat(&mut self, player_id: &str, beating: Card, beatable: Card) -> Result<i32, ()> {
        if !self.player_has_card(&player_id, beating) ||
            !self.table_has_open_card(beatable) ||
            !self.can_beat(beating, beatable) {
            return Err(());
        }

        let mut table_element_id = 0;

        for i in 0..self.table.len() {
            let (bottom, _) = self.table[i];
            if bottom == beatable {
                self.table[i].1 = Some(beating);
                self.pick_card(&player_id, beating)?;
                table_element_id = i as i32;
            }
        }

        Ok(table_element_id)
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

        self.drop_beat_confirmations();
        self.table = vec![];

        Ok(())
    }

    pub fn discard_table(&mut self) -> Result<(), ()> {
        for pair in &self.table {
            self.discard.push(pair.0);
            if pair.1.is_some() {
                self.discard.push(pair.1.unwrap());
            }
        }

        self.drop_beat_confirmations();
        self.table = vec![];

        Ok(())
    }

    pub fn deal_more(&mut self, defending_player_id: &str) -> Result<(), ()> {
        let defending_player_num = match self
            .hands_order.iter().position(|x| {x == defending_player_id}) {
            Some(res) => res,
            None => {
                return Err(());
            }
        };

        let attacking_player_num = if defending_player_num > 0
            { defending_player_num - 1 } else { self.hands_order.len() - 1 };

        let dealing_order: Vec<String> = [
            &self.hands_order[defending_player_num..], &self.hands_order[..=attacking_player_num]
        ].concat().into_iter().rev().collect();

        for player_id in dealing_order {
            self.deal_to_hand_until_full(&player_id).unwrap();
        }

        Ok(())
    }

    pub fn can_beat(&self, beating: Card, beatable: Card) -> bool {
        (beating.s == beatable.s && beating.r > beatable.r) ||
        (beating.s == self.trump_suit && beatable.s != self.trump_suit)
    }

    pub fn can_discard(&self, target_id: &str) -> bool {
        self.is_all_confirmed(target_id)
    }

    pub fn drop_beat_confirmations(&mut self) {
        for key in self.beat_confirmations.values_mut() {
            *key = true;
        }
    }

    pub fn get_min_trump(&self, player_id: &str) -> Card {
        let hand = self.hands.get(player_id).unwrap();

        let min_trump = hand.iter()
            .filter(|x| { x.s == self.trump_suit })
            .reduce(|a, b| { if a.s > b.s {a} else {b} });

        match min_trump {
            Some(card) => *card,
            None => Card::new(0, self.trump_suit, 0)
        }
    }

    pub fn is_table_beaten(&self) -> bool {
        for (_, top_card) in &self.table {
            if top_card.is_none() {
                return false;
            }
        }
        true
    }

    pub fn player_after(&self, player_id: &str) -> Option<String> {
        let player_index = self.hands_order.iter().position(|item| {item == player_id});

        if player_index.is_none() {
            return None
        }

        let next_index = (player_index.unwrap() + 1) % &self.hands_amount;

        Some(self.hands_order[next_index].clone())
    }

    pub fn player_before(&self, player_id: &str) -> Option<String> {
        let mut result = self.player_after(player_id);

        if result.is_none() {
            return None
        }

        let mut result = result.unwrap();

        for i in 0..self.hands.len() - 2 {
            result = self.player_after(&result).unwrap();
        }

        Some(result)
    }

    pub fn get_first_target_player(&self) -> String {
        self.player_after(&self.hands_order[0]).unwrap()
    }

    pub fn get_first_attacker_player(&self) -> String {
        self.hands_order[0].clone()
    }

    pub fn confirm_beat(&mut self, player_id: String)-> Result<(), ()> {
        if !self.beat_confirmations.contains_key(&player_id) {
            return Err(())
        }

        self.beat_confirmations.insert(player_id, true);

        Ok(())
    }

    pub fn is_all_confirmed(&self, target_player: &str) -> bool {
        let mut result = true;
        for key in self.beat_confirmations.keys() {
            if (key != target_player) {
                result = result && *self.beat_confirmations.get(key).unwrap();
            }
        }
        return result
    }

    pub fn get_table_size(&self) -> usize {
        self.table.len()
    }

    pub fn get_table_element_cards(&self, element_id: usize) -> Vec<Card> {
        let mut result = vec![self.table[element_id].0];

        if (self.table[element_id].1.is_some()) {
            result.push(self.table[element_id].1.unwrap());
        }

        result
    }

    fn init_order(&mut self) {
        let mut hands_in_order = Vec::<(String)>::new();

        for key in self.hands.keys().into_iter() {
            hands_in_order.push(key.to_string());
        }

        hands_in_order.sort_by(|a, b| {
            self.get_min_trump(a).r.cmp(&self.get_min_trump(b).r)
        });

        self.hands_order = hands_in_order;
    }

    fn deal_to_hand_until_full(&mut self, player_id: &str) -> Result<(), ()> {
        let hand = match self.hands.get_mut(player_id) {
            Some(res) => res,
            None => {
                return Err(());
            }
        };

        while hand.len() < self.hand_size && self.deck.len() > 0 {
            hand.push(self.deck.pop().unwrap());
        }

        Ok(())
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
        match self.hands.get(player_id) {
            Some(res) => res,
            None => {
                return false;
            }
        }.contains(&card)
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
            if bottom.r == card.r {
                return true;
            }
            if let Some(top) = top {
                if top.r == card.r {
                    return true;
                }
            }
        }
        false
    }
}
