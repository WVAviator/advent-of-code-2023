use std::fs::File;

mod camel_cards;
mod cube_conundrum;
mod gear_ratios;
mod haunted_wasteland;
mod if_you_give_a_seed_a_fertilizer;
mod scratchcards;
mod trebuchet;
mod wait_for_it;

pub trait Challenge {
    fn solve_part_one(&self) -> String;
    fn solve_part_two(&self) -> String;
    fn load(&mut self, file: &File);
}

pub struct ChallengeFactory;

impl ChallengeFactory {
    pub fn create(challenge_id: &u8, file: &File) -> Box<dyn Challenge> {
        let mut solution: Box<dyn Challenge> = match challenge_id {
            1 => Box::new(trebuchet::Trebuchet::default()),
            2 => Box::new(cube_conundrum::CubeConundrum::default()),
            3 => Box::new(gear_ratios::GearRatios::default()),
            4 => Box::new(scratchcards::Scratchcards::default()),
            5 => Box::new(if_you_give_a_seed_a_fertilizer::IfYouGiveASeedAFertilizer::default()),
            6 => Box::new(wait_for_it::WaitForIt::default()),
            7 => Box::new(camel_cards::CamelCards::default()),
            8 => Box::new(haunted_wasteland::HauntedWasteland::default()),
            _ => panic!("Challenge {} not implemented yet!", challenge_id),
        };

        solution.load(file);
        solution
    }
}
