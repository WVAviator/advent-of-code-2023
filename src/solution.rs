use std::fs::File;

mod solution1;

pub trait Solution {
    fn solve(&self) -> String;
    fn load(&mut self, file: &File);
}

pub struct SolutionFactory;

impl SolutionFactory {
    pub fn create(challenge_id: &u8, file: &File) -> Box<dyn Solution> {
        let mut solution = match challenge_id {
            1 => Box::new(solution1::Solution1::default()),
            _ => panic!("Challenge {} not implemented yet!", challenge_id),
        };

        solution.load(file);
        solution
    }
}
