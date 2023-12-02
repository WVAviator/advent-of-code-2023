use super::cube_subset::CubeSubset;
use std::cmp::max;

#[derive(Debug, PartialEq)]
pub struct CubeGame {
    pub id: u32,
    subsets: Vec<CubeSubset>,
}

impl CubeGame {
    pub fn new(line: &str) -> Self {
        let id = CubeGame::extract_game_id(line);
        let subsets = CubeGame::extract_game_segments(line);
        CubeGame { subsets, id }
    }

    pub fn is_possible(&self, max_red: u32, max_green: u32, max_blue: u32) -> bool {
        for subset in &self.subsets {
            if subset.red > max_red || subset.green > max_green || subset.blue > max_blue {
                return false;
            }
        }
        return true;
    }

    pub fn get_power(&self) -> u32 {
        let (red, green, blue) = self.min_required();
        let power = red * green * blue;

        power
    }

    fn min_required(&self) -> (u32, u32, u32) {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for subset in &self.subsets {
            red = max(red, subset.red);
            green = max(green, subset.green);
            blue = max(blue, subset.blue);
        }
        (red, green, blue)
    }

    fn extract_game_id(line: &str) -> u32 {
        let game_id_segment = line.split(":").next().expect(
            format!(
                "Could not identify string segment before ':' in line: {}",
                line
            )
            .as_str(),
        );
        game_id_segment
            .split(" ")
            .last()
            .expect(
                format!(
                    "Could not identify value after ' ' in game id segment: {}",
                    game_id_segment
                )
                .as_str(),
            )
            .parse::<u32>()
            .expect(
                format!(
                    "Could not parse u32 game ID value from line segment {}",
                    game_id_segment
                )
                .as_str(),
            )
    }

    fn extract_game_segments(line: &str) -> Vec<CubeSubset> {
        let game_segments = line.split(":").skip(1).next().expect(
            format!(
                "Could not identify string segment after ':' in line: {}",
                line
            )
            .as_str(),
        );
        let game_segments = game_segments.split(";").collect::<Vec<&str>>();
        let mut subsets = Vec::new();
        for segment in game_segments {
            let subset = CubeSubset::parse(segment);
            subsets.push(subset);
        }
        subsets
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ch02_cubegame_extract_game_id_single_digit() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game_id = CubeGame::extract_game_id(line);
        assert_eq!(game_id, 1);
    }

    #[test]
    fn ch02_cubegame_extract_game_id_double_digit() {
        let line = "Game 93: 1 blue, 6 red; 2 blue, 10 red, 2 green; 2 green, 2 blue; 4 red, 2 blue; 4 red, 3 green";
        let game_id = CubeGame::extract_game_id(line);
        assert_eq!(game_id, 93);
    }

    #[test]
    fn ch02_cubegame_extract_game_id_triple_digit() {
        let line = "Game 100: 1 blue, 6 red; 2 blue, 10 red, 2 green; 2 green, 2 blue; 4 red, 2 blue; 4 red, 3 green";
        let game_id = CubeGame::extract_game_id(line);
        assert_eq!(game_id, 100);
    }

    #[test]
    fn ch02_cubegame_extract_game_segments() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let subsets = CubeGame::extract_game_segments(line);
        assert_eq!(subsets.len(), 3);
        let expected_subsets = vec![
            CubeSubset::parse("3 blue, 4 red"),
            CubeSubset::parse("1 red, 2 green, 6 blue"),
            CubeSubset::parse("2 green"),
        ];

        assert_eq!(subsets, expected_subsets);
    }

    #[test]
    fn ch02_cubegame_parse() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = CubeGame::new(line);
        assert_eq!(game.id, 1);
        assert_eq!(game.subsets.len(), 3);

        let expected = vec![
            CubeSubset::parse("3 blue, 4 red"),
            CubeSubset::parse("1 red, 2 green, 6 blue"),
            CubeSubset::parse("2 green"),
        ];

        assert_eq!(game.subsets, expected);
    }

    #[test]
    fn ch02_cubegame_min_required() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = CubeGame::new(line);
        let (red, green, blue) = game.min_required();
        assert_eq!(red, 4);
        assert_eq!(green, 2);
        assert_eq!(blue, 6);
    }

    #[test]
    fn ch02_cubegame_get_power() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = CubeGame::new(line);
        let power = game.get_power();
        assert_eq!(power, 48);
    }

    #[test]
    fn ch02_cubegame_get_power_2() {
        let line = "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red";
        let game = CubeGame::new(line);
        let power = game.get_power();
        assert_eq!(power, 1560);
    }
}
