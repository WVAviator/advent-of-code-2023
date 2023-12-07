use num::BigInt;

use crate::utils::to_lines_vec;

use self::hand::Hand;

use super::Challenge;
use std::fs::File;

mod card;
mod hand;
mod hand_type;

#[derive(Default)]
pub struct CamelCards {
    hands: Vec<Hand>,
}

impl Challenge for CamelCards {
    fn load(&mut self, file: &File) {
        self.load_hands(to_lines_vec(file));
    }
    fn solve_part_one(&self) -> String {
        let mut hands = self.hands.clone();
        hands.sort();
        let result: BigInt = hands
            .iter()
            .enumerate()
            .fold(BigInt::from(0), |acc, (i, hand)| {
                let winnings = hand.bid as usize * (i + 1);
                acc + BigInt::from(winnings)
            });

        format!("{}", result)
    }
    fn solve_part_two(&self) -> String {
        format!("Not implemented yet!")
    }
}

impl CamelCards {
    fn load_hands(&mut self, lines: Vec<String>) {
        self.hands = lines.into_iter().map(|line| Hand::parse(line)).collect();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn get_input() -> Vec<String> {
        vec![
            String::from("32T3K 765"),
            String::from("T55J5 684"),
            String::from("KK677 28"),
            String::from("KTJJT 220"),
            String::from("QQQJA 483"),
        ]
    }

    #[test]
    fn ch07_camel_cards_part_one() {
        let mut camel_cards = CamelCards::default();
        camel_cards.load_hands(get_input());

        assert_eq!(camel_cards.solve_part_one(), "6440");
    }
}
