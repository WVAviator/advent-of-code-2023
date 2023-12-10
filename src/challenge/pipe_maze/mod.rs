use crate::utils::to_lines_vec;

use self::pipe_map::PipeMap;

use super::Challenge;
use std::fs::File;

mod pipe;
mod pipe_map;
mod pipe_type;

#[derive(Default)]
pub struct PipeMaze {
    pipe_map: PipeMap,
}

impl Challenge for PipeMaze {
    fn load(&mut self, file: &File) {
        self.load_map(to_lines_vec(file));
    }
    fn solve_part_one(&self) -> String {
        let furthest = self
            .pipe_map
            .into_iter()
            .enumerate()
            .zip(self.pipe_map.into_iter().rev().enumerate())
            .filter(|((_, p1), (_, p2))| p1 == p2)
            .map(|((i1, _), (_, _))| i1)
            .max()
            .unwrap();
        format!("{}", furthest)
    }
    fn solve_part_two(&self) -> String {
        format!("Not implemented yet!")
    }
}

impl PipeMaze {
    pub fn load_map(&mut self, lines: Vec<String>) {
        self.pipe_map = PipeMap::new(lines);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ch10_pipe_maze_part_one_example_one() {
        let lines = vec![
            String::from("....."),
            String::from(".S-7."),
            String::from(".|.|."),
            String::from(".L-J."),
            String::from("....."),
        ];

        let mut pipe_maze = PipeMaze::default();
        pipe_maze.load_map(lines);

        assert_eq!(pipe_maze.solve_part_one(), "4");
    }

    #[test]
    fn ch10_pipe_maze_part_one_example_two() {
        let lines = vec![
            String::from("..F7."),
            String::from(".FJ|."),
            String::from("SJ.L7"),
            String::from("|F--J"),
            String::from("LJ..."),
        ];

        let mut pipe_maze = PipeMaze::default();
        pipe_maze.load_map(lines);

        assert_eq!(pipe_maze.solve_part_one(), "8");
    }
}
