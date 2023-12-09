pub struct History {
    sequence: Vec<i32>,
}

impl History {
    pub fn new(line: &String) -> Self {
        let sequence = line
            .split(' ')
            .map(|v| v.parse::<i32>().expect("Could not convert to number."))
            .collect();
        History { sequence }
    }

    pub fn extrapolate_next(&self) -> i32 {
        let mut rows = Vec::new();
        let mut next_row: Vec<i32> = self.sequence.clone();
        rows.push(next_row.clone());

        loop {
            next_row = next_row
                .iter()
                .skip(1)
                .zip(next_row.iter())
                .map(|(p, c)| p - c)
                .collect();
            rows.push(next_row.clone());
            if next_row.iter().all(|v| v.eq(&0)) {
                break;
            }
        }

        rows.iter()
            .rev()
            .map(|row| row.last().expect("No last element in row found."))
            .fold(0, |acc, cur| acc + cur)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ch09_history_extrapolate_next() {
        let history = History::new(&String::from("3 6 9 12 15"));
        assert_eq!(history.extrapolate_next(), 18);
    }

    #[test]
    fn ch09_history_extrapolate_next_ascending() {
        let history = History::new(&String::from("1 3 6 10 15 21"));
        assert_eq!(history.extrapolate_next(), 28);
    }
}
