use super::resource::Resource;

pub struct ResourceMap {
    pub from: Resource,
    to: Resource,
    ranges: Vec<MapRange>,
}

impl ResourceMap {
    pub fn new(lines: Vec<String>) -> Self {
        let mut line_iter = lines.iter();

        let mut first_line_iter = line_iter
            .next()
            .expect("No lines to extract.")
            .split(' ')
            .next()
            .expect("Error extracting map type.")
            .split("-to-");
        let from_str = first_line_iter
            .next()
            .expect("Could not extract 'from' value from string.");
        let to_str = first_line_iter
            .next()
            .expect("Could not extract 'to' from string.");

        let ranges = line_iter.map(|line| MapRange::from(line)).collect();

        ResourceMap {
            from: Resource::from(from_str),
            to: Resource::from(to_str),
            ranges,
        }
    }

    pub fn map_to(&self, from: u32) -> (u32, Resource) {
        let mut to = from;
        for range in &self.ranges {
            if range.contains(from.into()) {
                to = range.map_to(from.into()) as u32;
            }
        }

        (to, self.to.clone())
    }

    pub fn get_from(&self) -> Resource {
        self.from.clone()
    }
}

#[derive(PartialEq, Debug)]
struct MapRange {
    from_start: u64,
    to_start: u64,
    length: u64,
}

impl MapRange {
    pub fn from(line: &str) -> Self {
        let mut values = line.split(' ');

        MapRange {
            to_start: values
                .next()
                .expect("Invalid MapRange string.")
                .parse()
                .expect("Could not parse range value into number."),
            from_start: values
                .next()
                .expect("Invalid MapRange string.")
                .parse()
                .expect("Could not parse range value into number."),
            length: values
                .next()
                .expect("Invalid MapRange string.")
                .parse()
                .expect("Could not parse range value into number."),
        }
    }

    pub fn contains(&self, value: u64) -> bool {
        let range = self.from_start..=(self.from_start + self.length);
        range.contains(&value)
    }

    pub fn map_to(&self, from: u64) -> u64 {
        if !self.contains(from) {
            panic!("Attempted to map a value that is not in the range.");
        }
        let from_diff = from - self.from_start;
        self.to_start + from_diff
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ch05_map_range_from() {
        let line = "45 77 23";
        let map_range = MapRange::from(line);

        assert_eq!(map_range.from_start, 77);
        assert_eq!(map_range.to_start, 45);
        assert_eq!(map_range.length, 23);
    }

    #[test]
    fn ch05_map_range_contains() {
        let line = "45 77 23";
        let map_range = MapRange::from(line);

        assert!(map_range.contains(77));
        assert!(map_range.contains(100));
        assert!(map_range.contains(87));
        assert!(!map_range.contains(76));
        assert!(!map_range.contains(101));
    }

    #[test]
    fn ch05_map_range_map_to() {
        let line = "45 77 23";
        let map_range = MapRange::from(line);

        assert_eq!(map_range.map_to(78), 46);
    }

    #[test]
    fn ch05_resource_map_new() {
        let lines = vec![
            "soil-to-fertilizer map:  ".into(),
            "0 15 37".into(),
            "37 52 2".into(),
            "39 0 15".into(),
        ];

        let resource_map = ResourceMap::new(lines);

        let expected_ranges = vec![
            MapRange::from("0 15 37"),
            MapRange::from("37 52 2"),
            MapRange::from("39 0 15"),
        ];

        assert_eq!(resource_map.from, Resource::Soil);
        assert_eq!(resource_map.to, Resource::Fertilizer);
        assert_eq!(resource_map.ranges, expected_ranges);
    }
}
