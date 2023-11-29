use crate::utils::to_lines_vec;

use super::Solution;
use std::fs::File;

#[derive(Default)]
pub struct Solution1 {
    data: String,
}

impl Solution for Solution1 {
    fn load(&mut self, file: &File) {
        self.data = to_lines_vec(file).join("\n");
    }
    fn solve(&self) -> String {
        self.data.clone()
    }
}
