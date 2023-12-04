use super::scratchcard::Scratchcard;

pub trait ScoreCalculator {
    fn calculate_card(scratchcard: &Scratchcard) -> u32;
    fn calculate_total(scratchcards: &Vec<Scratchcard>) -> u32;
}
