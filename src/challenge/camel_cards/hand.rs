use super::{card::Card, hand_type::HandType};

#[derive(Debug, Eq, PartialEq, Ord, Clone)]
pub struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
    pub bid: u32,
}

impl Hand {
    pub fn parse(line: String, with_joker: bool) -> Self {
        let mut line_iter = line.split(' ');
        let cards = line_iter
            .next()
            .expect("Could not find hand string.")
            .chars()
            .map(|c| {
                if with_joker {
                    return Card::from_with_joker(c);
                }
                Card::from(c)
            })
            .collect::<Vec<Card>>();
        let bid = line_iter
            .next()
            .expect("Could not find bid string.")
            .parse::<u32>()
            .expect("Could not convert bid to number.");
        let hand_type = HandType::parse(&cards);

        Hand {
            cards,
            hand_type,
            bid,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.hand_type.partial_cmp(&other.hand_type) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        match self.cards.iter().partial_cmp(other.cards.iter()) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        Some(core::cmp::Ordering::Equal)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ch07_hand_parse() {
        let hand = Hand::parse(String::from("AAKK5 123"), false);
        assert_eq!(
            hand,
            Hand {
                cards: vec![
                    Card::from('A'),
                    Card::from('A'),
                    Card::from('K'),
                    Card::from('K'),
                    Card::from('5')
                ],
                hand_type: HandType::TwoPair,
                bid: 123
            }
        );
    }

    #[test]
    fn ch07_hand_ord() {
        let mut hands = vec![
            Hand::parse(String::from("AAKK5 123"), false),
            Hand::parse(String::from("TTTT3 123"), false),
            Hand::parse(String::from("KKJJ4 123"), false),
            Hand::parse(String::from("KK234 123"), false),
            Hand::parse(String::from("2AA34 123"), false),
        ];

        hands.sort();

        let expected = vec![
            Hand::parse(String::from("2AA34 123"), false),
            Hand::parse(String::from("KK234 123"), false),
            Hand::parse(String::from("KKJJ4 123"), false),
            Hand::parse(String::from("AAKK5 123"), false),
            Hand::parse(String::from("TTTT3 123"), false),
        ];

        assert_eq!(hands, expected);

        assert!(
            Hand::parse(String::from("KKQQQ 123"), false)
                > Hand::parse(String::from("QQJJJ 123"), false)
        );
        assert!(
            Hand::parse(String::from("QQJJJ 123"), false)
                > Hand::parse(String::from("KKKJ5 123"), false)
        );
    }
}
