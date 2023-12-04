use std::collections::HashSet;

#[derive(Debug, PartialEq)]
pub struct Scratchcard {
    pub id: u32,
    winning_numbers: HashSet<u32>,
    candidate_numbers: Vec<u32>,
}

impl Scratchcard {
    pub fn parse(line: &str) -> Self {
        let line_err = format!("Error parsing line: {}", line);
        let mut line_iter = line.split(':');
        let id = line_iter
            .next()
            .expect(&line_err)
            .split(' ')
            .filter(|v| !v.is_empty())
            .skip(1)
            .next()
            .expect(&line_err)
            .parse::<u32>()
            .expect(&line_err);

        let mut num_iter = line_iter.next().expect(&line_err).split('|');

        let winning_numbers: HashSet<u32> = num_iter
            .next()
            .expect(&line_err)
            .split(' ')
            .filter(|v| !v.is_empty())
            .map(|v| v.trim().parse::<u32>().expect(&line_err))
            .collect();

        let candidate_numbers: Vec<u32> = num_iter
            .next()
            .expect(&line_err)
            .split(' ')
            .filter(|v| !v.is_empty())
            .map(|v| v.trim().parse::<u32>().expect(&line_err))
            .collect();

        Scratchcard {
            id,
            winning_numbers,
            candidate_numbers,
        }
    }

    pub fn get_matches(&self) -> Vec<&u32> {
        self.candidate_numbers
            .iter()
            .filter(|v| self.winning_numbers.contains(v))
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ch04_scratchcard_parse() {
        let test_card = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let scratchcard = Scratchcard::parse(test_card);
        let expected = Scratchcard {
            id: 1,
            winning_numbers: HashSet::from([41, 48, 83, 86, 17]),
            candidate_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
        };
        assert_eq!(scratchcard, expected);
    }

    #[test]
    fn ch04_scratchcard_parse_with_spaces() {
        let test_card = "Card     1: 41   48 83 86   17     | 83   86  6 31 17  9     48 53   ";
        let scratchcard = Scratchcard::parse(test_card);
        let expected = Scratchcard {
            id: 1,
            winning_numbers: HashSet::from([41, 48, 83, 86, 17]),
            candidate_numbers: vec![83, 86, 6, 31, 17, 9, 48, 53],
        };
        assert_eq!(scratchcard, expected);
    }
}
