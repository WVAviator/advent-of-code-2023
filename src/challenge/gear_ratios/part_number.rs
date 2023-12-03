#[derive(Clone, Copy, PartialEq, Debug)]
pub struct PartNumber {
    pub row: usize,
    pub col: usize,
    pub length: usize,
    pub value: u32,
}

impl PartNumber {
    pub fn new(row_id: usize, row: &Vec<char>, start: usize) -> Result<Self, ()> {
        if !row[start].is_digit(10) {
            return Err(());
        }
        let num_str: String = row
            .iter()
            .skip(start)
            .take_while(|ch| ch.is_digit(10))
            .collect();
        let value = num_str
            .parse::<u32>()
            .expect(format!("Attempted to parse non-numeric char sequence: {}", num_str).as_str());

        Ok(PartNumber {
            row: row_id,
            col: start,
            length: num_str.len(),
            value,
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ch03_part_number_new() {
        let row = vec!['.', '.', '4', '6', '1', '.', '7', '6', '.'];
        let expected = PartNumber {
            row: 4,
            col: 2,
            length: 3,
            value: 461,
        };

        assert_eq!(expected, PartNumber::new(4, &row, 2).unwrap());
    }

    #[test]
    fn ch03_part_number_symbol() {
        let row = vec!['.', '.', '4', '6', '1', '$', '7', '6', '#'];
        let expected = PartNumber {
            row: 4,
            col: 6,
            length: 2,
            value: 76,
        };

        assert_eq!(expected, PartNumber::new(4, &row, 6).unwrap());
    }

    #[test]
    fn ch03_part_number_errs() {
        let row = vec!['.', '.', '4', '6', '1', '.', '7', '6', '.'];

        assert_eq!(Err(()), PartNumber::new(4, &row, 1));
    }
}
