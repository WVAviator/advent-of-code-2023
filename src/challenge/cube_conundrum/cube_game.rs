use super::cube_subset::CubeSubset;

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
}
