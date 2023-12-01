use crate::utils::to_lines_vec;

use super::Challenge;
use std::fs::File;

#[derive(Default)]
pub struct Trebuchet {
    data: String,
}

impl Challenge for Trebuchet {
    fn load(&mut self, file: &File) {
        self.data = to_lines_vec(file).join("\n");
    }
    fn solvePartOne(&self) -> String {
        format!("Not implemented yet!")
    }
    fn solvePartTwo(&self) -> String {
        format!("Not implemented yet!")
    }
}