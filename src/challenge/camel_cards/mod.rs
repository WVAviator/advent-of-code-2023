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
    lines: Vec<String>,
}

impl Challenge for CamelCards {
    fn load(&mut self, file: &File) {
        self.lines = to_lines_vec(file);
    }
    fn solve_part_one(&self) -> String {
        let mut hands: Vec<Hand> = self
            .lines
            .iter()
            .map(|line| Hand::parse(line.clone(), false))
            .collect();
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
        let mut hands: Vec<Hand> = self
            .lines
            .iter()
            .map(|line| Hand::parse(line.clone(), true))
            .collect();
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
        camel_cards.lines = get_input();

        assert_eq!(camel_cards.solve_part_one(), "6440");
    }

    #[test]
    fn ch07_camel_cards_part_two() {
        let mut camel_cards = CamelCards::default();
        camel_cards.lines = get_input();

        assert_eq!(camel_cards.solve_part_two(), "5905");
    }
}
