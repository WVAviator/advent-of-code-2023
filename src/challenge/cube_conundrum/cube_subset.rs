#[derive(Debug, PartialEq)]

pub struct CubeSubset {
    red: u32,
    green: u32,
    blue: u32,
}

impl CubeSubset {
    pub fn parse(segment: &str) -> Self {
        let colors = segment.split(",").collect::<Vec<&str>>();
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        // " 3 blue, 4 red"
        for color in colors {
            let color = color.trim();
            let mut color_iter = color.split(" ");
            let count = color_iter
                .next()
                .expect(format!("Could not identify count in color: {}", color).as_str())
                .parse::<u32>()
                .expect(format!("Could not parse u8 count in color: {}", color).as_str());
            let color = color_iter
                .next()
                .expect(format!("Could not identify color in color: {}", color).as_str());
            match color {
                "red" => red = count as u32,
                "green" => green = count as u32,
                "blue" => blue = count as u32,
                _ => panic!("Unknown color: {}", color),
            }
        }

        CubeSubset { red, green, blue }
    }
}

mod test {
    use super::*;

    #[test]
    fn ch02_cubesubset_parse() {
        let segment = " 3 blue, 4 red";
        let subset = CubeSubset::parse(segment);
        assert_eq!(subset.red, 4);
        assert_eq!(subset.green, 0);
        assert_eq!(subset.blue, 3);
    }

    #[test]
    fn ch02_cubesubset_parse_multiple_colors() {
        let segment = " 3 blue, 4 red, 2 green";
        let subset = CubeSubset::parse(segment);
        assert_eq!(subset.red, 4);
        assert_eq!(subset.green, 2);
        assert_eq!(subset.blue, 3);
    }

    #[test]
    fn ch02_cubesubset_parse_multiple_colors_unordered() {
        let segment = " 3 blue, 4 red, 2 green";
        let subset = CubeSubset::parse(segment);
        assert_eq!(subset.red, 4);
        assert_eq!(subset.green, 2);
        assert_eq!(subset.blue, 3);
    }

    #[test]
    fn ch02_cubesubset_parse_one_color() {
        let segment = " 3 blue";
        let subset = CubeSubset::parse(segment);
        assert_eq!(subset.red, 0);
        assert_eq!(subset.green, 0);
        assert_eq!(subset.blue, 3);
    }
}
