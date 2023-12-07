use std::collections::HashMap;

use super::card::Card;

#[derive(Eq, PartialEq, Debug, PartialOrd, Ord, Clone, Copy)]
pub enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    pub fn parse(hand: &Vec<Card>) -> Self {
        let mut counts: HashMap<&Card, u8> = HashMap::new();
        for card in hand {
            *counts.entry(card).or_insert(0) += 1;
        }

        match counts.len() {
            5 => HandType::HighCard,
            4 => HandType::OnePair,
            3 => {
                if counts.iter().any(|(_, v)| *v == 3) {
                    return HandType::ThreeOfAKind;
                }
                HandType::TwoPair
            }
            2 => {
                if counts.iter().any(|(_, v)| *v == 4) {
                    return HandType::FourOfAKind;
                }
                HandType::FullHouse
            }
            1 => HandType::FiveOfAKind,
            _ => panic!("Invalid hand."),
        }
    }
}

#[cfg(test)]
mod test {
    use crate::challenge::camel_cards::card::Card;

    use super::*;

    fn get_cards(hand: &str) -> Vec<Card> {
        hand.chars().map(|c| Card::from(c)).collect::<Vec<Card>>()
    }

    #[test]
    fn ch07_hand_type_parse() {
        let hand = get_cards("KKTT4");
        assert_eq!(HandType::parse(&hand), HandType::TwoPair);

        let hand = get_cards("23456");
        assert_eq!(HandType::parse(&hand), HandType::HighCard);

        let hand = get_cards("AAAA4");
        assert_eq!(HandType::parse(&hand), HandType::FourOfAKind);

        let hand = get_cards("KKK65");
        assert_eq!(HandType::parse(&hand), HandType::ThreeOfAKind);

        let hand = get_cards("KKJJJ");
        assert_eq!(HandType::parse(&hand), HandType::FullHouse);
    }

    #[test]
    fn ch07_hand_type_ord() {
        assert!(HandType::FourOfAKind > HandType::TwoPair);
        assert!(HandType::FiveOfAKind > HandType::FullHouse);
        assert!(HandType::FullHouse > HandType::HighCard);
        assert!(HandType::ThreeOfAKind > HandType::TwoPair);
        assert!(HandType::FourOfAKind > HandType::OnePair);
    }
}
