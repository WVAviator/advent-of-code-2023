use crate::{challenge::mirage_maintenance::history::History, utils::to_lines_vec};

use super::Challenge;
use std::fs::File;

mod history;

#[derive(Default)]
pub struct MirageMaintenance {
    lines: Vec<String>,
}

impl Challenge for MirageMaintenance {
    fn load(&mut self, file: &File) {
        self.lines = to_lines_vec(file);
    }
    fn solve_part_one(&self) -> String {
        let histories_sum: i32 = self
            .lines
            .iter()
            .map(|line| History::new(line))
            .map(|history| history.extrapolate_next())
            .sum();

        format!("{}", histories_sum)
    }
    fn solve_part_two(&self) -> String {
        format!("Not implemented yet!")
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ch09_mirage_maintenance_part_one() {
        let lines = vec![
            String::from("0 3 6 9 12 15"),
            String::from("1 3 6 10 15 21"),
            String::from("10 13 16 21 30 45"),
        ];

        let mut mirage_maintenance = MirageMaintenance::default();
        mirage_maintenance.lines = lines;

        assert_eq!(mirage_maintenance.solve_part_one(), "114");
    }
}
