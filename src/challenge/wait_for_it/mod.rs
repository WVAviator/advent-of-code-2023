use crate::utils::to_lines_vec;

use self::race::RaceInfo;

use super::Challenge;
use std::fs::File;

mod race;

#[derive(Default)]
pub struct WaitForIt {
    races: Vec<RaceInfo>,
    lines: Vec<String>,
}

impl WaitForIt {
    pub fn load(&mut self, lines: Vec<String>) {
        self.lines = lines.clone();
        let mut lines_iter = lines.into_iter();
        let time_line = lines_iter.next().expect("Time line missing.");
        let distance_line = lines_iter.next().expect("Distance line missing.");

        self.races = time_line
            .split(' ')
            .filter(|x| !x.is_empty())
            .zip(distance_line.split(' ').filter(|x| !x.is_empty()))
            .skip(1)
            .map(|(time_str, dist_str)| {
                let time = time_str.parse().expect("Unable to parse time string.");
                let distance = dist_str.parse().expect("Unable to parse distanc string.");

                RaceInfo::new(time, distance)
            })
            .collect();
    }
}

impl Challenge for WaitForIt {
    fn load(&mut self, file: &File) {
        self.load(to_lines_vec(file));
    }
    fn solve_part_one(&self) -> String {
        let ways_to_win: usize = self
            .races
            .iter()
            .map(|race| race.all_winning_races().len())
            .product();

        format!("{}", ways_to_win)
    }
    fn solve_part_two(&self) -> String {
        let mut lines_iter = self.lines.iter();
        let time = lines_iter
            .next()
            .expect("Unable to parse time line.")
            .split(' ')
            .skip(1)
            .filter(|x| !x.is_empty())
            .fold(String::new(), |a, c| format!("{}{}", a, c))
            .parse::<u64>()
            .expect("Unable to parse time string.");
        let distance = lines_iter
            .next()
            .expect("Unable to parse distance line.")
            .split(' ')
            .skip(1)
            .filter(|x| !x.is_empty())
            .fold(String::new(), |a, c| format!("{}{}", a, c))
            .parse::<u64>()
            .expect("Unable to parse distance string.");

        let race = RaceInfo::new(time, distance);
        let ways_to_win = race.all_winning_races().len();

        format!("{}", ways_to_win)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ch06_waitforit_part_one() {
        let lines: Vec<String> = vec![
            "Time:      7  15   30".into(),
            "Distance:  9  40  200".into(),
        ];

        let mut wait_for_it = WaitForIt::default();
        wait_for_it.load(lines);

        assert_eq!(wait_for_it.solve_part_one(), "288");
    }

    #[test]
    fn ch06_waitforit_part_two() {
        let lines: Vec<String> = vec![
            "Time:      7  15   30".into(),
            "Distance:  9  40  200".into(),
        ];

        let mut wait_for_it = WaitForIt::default();
        wait_for_it.load(lines);

        assert_eq!(wait_for_it.solve_part_two(), "71503");
    }
}
