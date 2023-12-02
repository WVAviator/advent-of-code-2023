use crate::utils::to_lines_vec;

use self::cube_game::CubeGame;

use super::Challenge;
use std::fs::File;

mod cube_game;
mod cube_subset;

#[derive(Default)]
pub struct CubeConundrum {
    games: Vec<CubeGame>,
}

impl Challenge for CubeConundrum {
    fn load(&mut self, file: &File) {
        let lines: Vec<String> = to_lines_vec(file);
        let mut games: Vec<CubeGame> = Vec::new();
        lines.into_iter().for_each(|line| {
            games.push(CubeGame::new(&line));
        });

        self.games = games;
    }
    fn solve_part_one(&self) -> String {
        format!("Not implemented yet!")
    }
    fn solve_part_two(&self) -> String {
        format!("Not implemented yet!")
    }
}

impl CubeConundrum {}

mod test {
    use super::*;
}
