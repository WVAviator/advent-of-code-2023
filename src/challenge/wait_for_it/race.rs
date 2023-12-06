pub struct Race {
    pub hold_time: u64,
    pub travel_distance: u64,
}

impl Race {
    pub fn new(total_time: u64, hold_time: u64) -> Self {
        if hold_time >= total_time {
            panic!("Held the button down for too long, race failed.");
        }
        let travel_distance = hold_time * (total_time - hold_time);

        Race {
            hold_time,
            travel_distance,
        }
    }
}

pub struct RaceInfo {
    time: u64,
    record_distance: u64,
}

impl RaceInfo {
    pub fn new(time: u64, distance: u64) -> Self {
        RaceInfo {
            time,
            record_distance: distance,
        }
    }

    fn beats_record(&self, race: &Race) -> bool {
        race.travel_distance > self.record_distance
    }

    pub fn all_winning_races(&self) -> Vec<Race> {
        let mut result = Vec::new();
        for i in 1..self.time {
            let race = Race::new(self.time, i);
            if self.beats_record(&race) {
                result.push(race);
            }
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ch06_raceinfo_beats_record() {
        let race = Race::new(7, 3);
        assert_eq!(race.travel_distance, 12);

        let race_info = RaceInfo::new(7, 9);
        assert!(race_info.beats_record(&race));
    }

    #[test]
    fn ch06_raceinfo_all_winners() {
        let race_info = RaceInfo::new(7, 9);
        let winners = race_info.all_winning_races();

        assert_eq!(winners.len(), 4);
    }
}
