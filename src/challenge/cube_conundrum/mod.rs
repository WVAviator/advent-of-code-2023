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
        self.load_lines(lines);
    }
    fn solve_part_one(&self) -> String {
        let max_red = 12;
        let max_green = 13;
        let max_blue = 14;

        let possible_games_sum = self
            .games
            .iter()
            .filter(|game| game.is_possible(max_red, max_green, max_blue))
            .map(|game| game.id)
            .sum::<u32>();

        format!("{}", possible_games_sum)
    }
    fn solve_part_two(&self) -> String {
        format!("Not implemented yet!")
    }
}

impl CubeConundrum {
    fn load_lines(&mut self, lines: Vec<String>) {
        let mut games: Vec<CubeGame> = Vec::new();
        lines.into_iter().for_each(|line| {
            games.push(CubeGame::new(&line));
        });

        self.games = games;
    }
}

mod test {
    use super::*;

    #[test]
    fn ch02_part_one() {
        let test_lines = vec![
            String::from("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
            String::from("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
            String::from(
                "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            ),
            String::from(
                "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            ),
            String::from("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
        ];

        let mut cube_conundrum = CubeConundrum::default();
        cube_conundrum.load_lines(test_lines);

        assert_eq!(cube_conundrum.solve_part_one(), "8");
    }
}
