pub struct History {
    extrapolated_rows: Vec<Vec<i32>>,
}

impl History {
    pub fn new(line: &String) -> Self {
        let sequence = line
            .split(' ')
            .map(|v| v.parse::<i32>().expect("Could not convert to number."))
            .collect();
        let extrapolated_rows = History::extrapolate_rows(sequence);
        History { extrapolated_rows }
    }

    fn extrapolate_rows(sequence: Vec<i32>) -> Vec<Vec<i32>> {
        let mut rows = Vec::new();
        let mut next_row: Vec<i32> = sequence;
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

        rows
    }

    pub fn extrapolate_next(&self) -> i32 {
        self.extrapolated_rows
            .iter()
            .rev()
            .map(|row| row.last().expect("No last element in row found."))
            .fold(0, |acc, cur| acc + cur)
    }

    pub fn extrapolate_prev(&self) -> i32 {
        self.extrapolated_rows
            .iter()
            .rev()
            .map(|row| row.first().expect("No first element found in row."))
            .fold(0, |acc, cur| cur - acc)
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

    #[test]
    fn ch09_history_extrapolate_prev() {
        let history = History::new(&String::from("10 13 16 21 30 45"));
        assert_eq!(history.extrapolate_prev(), 5);
    }
}
