use crate::utils::to_lines_vec;

use super::Challenge;
use std::fs::File;

#[derive(Default)]
pub struct Scratchcards {
    data: String,
}

impl Challenge for Scratchcards {
    fn load(&mut self, file: &File) {
        self.data = to_lines_vec(file).join("\n");
    }
    fn solve_part_one(&self) -> String {
        format!("Not implemented yet!")
    }
    fn solve_part_two(&self) -> String {
        format!("Not implemented yet!")
    }
}
