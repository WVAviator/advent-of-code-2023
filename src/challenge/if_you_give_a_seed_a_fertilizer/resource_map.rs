use super::resource::Resource;

#[derive(Debug, Clone)]
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

    pub fn map_to(&self, from: u64) -> (u64, Resource) {
        let mut to = from;
        for range in &self.ranges {
            if range.contains(from) {
                to = range.map_to(from);
            }
        }

        (to, self.to.clone())
    }

    pub fn get_from(&self) -> Resource {
        self.from.clone()
    }

    pub fn get_to(&self) -> Resource {
        self.to.clone()
    }

    pub fn get_ranges(&self) -> &Vec<MapRange> {
        &self.ranges
    }

    pub fn merge_maps(&mut self, previous: &ResourceMap) {
        self.from = previous.get_from();
        self.normalize(previous);
        self.ranges.iter_mut().for_each(|range| {
            for prev_range in previous.get_ranges() {
                if prev_range.contains_to(range.from_start) {
                    range.add(&prev_range.get_diff());
                }
            }
        })
    }

    fn normalize(&mut self, previous: &ResourceMap) {
        let mut split_values: Vec<u64> = previous
            .ranges
            .iter()
            .flat_map(|range| [range.from_start, (range.from_start + range.length)].into_iter())
            .collect();
        split_values.sort();
        let new_ranges = self
            .ranges
            .iter()
            .flat_map(|range| {
                let mut parts = Vec::new();
                let mut right = *range;
                let mut i = 0;
                while i < split_values.len() {
                    if let Some((l, r)) = right.split_on(split_values[i]) {
                        right = r;
                        parts.push(l)
                    }

                    i += 1;
                }
                parts.push(right);
                parts.into_iter()
            })
            .collect();

        self.ranges = new_ranges;
    }

    pub fn lowest_overlap(&self, start: u64, length: u64) -> u64 {
        let map_range = MapRange {
            from_start: start,
            to_start: start,
            length,
        };
        let mut split_values: Vec<u64> = self
            .ranges
            .iter()
            .flat_map(|range| [range.from_start, (range.from_start + range.length)].into_iter())
            .collect();
        [map_range]
            .iter()
            .flat_map(|range| {
                let mut parts = Vec::new();
                let mut right = *range;
                let mut i = 0;
                while i < split_values.len() {
                    if let Some((l, r)) = right.split_on(split_values[i]) {
                        right = r;
                        parts.push(l)
                    }

                    i += 1;
                }
                parts.push(right);
                parts.into_iter()
            })
            .map(|range| self.map_to(range.from_start).0)
            .min()
            .expect("No values to calculate minimum in map range.")
    }
}

#[derive(PartialEq, Debug, Copy, Clone, PartialOrd, Ord, Eq)]
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
        let range = self.from_start..(self.from_start + self.length);
        range.contains(&value)
    }

    pub fn contains_to(&self, value: u64) -> bool {
        let range = self.to_start..(self.to_start + self.length);
        range.contains(&value)
    }

    pub fn get_diff(&self) -> i64 {
        self.to_start as i64 - self.from_start as i64
    }

    pub fn map_to(&self, from: u64) -> u64 {
        if !self.contains(from) {
            panic!("Attempted to map a value that is not in the range.");
        }
        let from_diff = from - self.from_start;
        self.to_start + from_diff
    }

    pub fn can_split(&self, value: u64) -> bool {
        self.contains(value) && value != self.from_start && value != self.from_start + self.length
    }

    pub fn split_on(&self, value: u64) -> Option<(Self, Self)> {
        if !self.can_split(value) {
            return None;
        }

        let left = MapRange {
            from_start: self.from_start,
            to_start: self.to_start,
            length: value - self.from_start,
        };
        let right = MapRange {
            from_start: value,
            to_start: self.to_start + (value - self.from_start),
            length: (self.from_start + self.length) - value,
        };

        Some((left, right))
    }

    pub fn add(&mut self, value: &i64) {
        let mut start = self.from_start as i64;
        start += value;
        self.from_start = start as u64;
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
    fn ch05_map_range_split_on() {
        let line = "45 77 23";
        let map_range = MapRange::from(line);
        let (left, right) = map_range.split_on(80).unwrap();

        assert_eq!(left.from_start, 77);
        assert_eq!(left.length, 3);
        assert_eq!(left.to_start, 45);
        assert_eq!(right.from_start, 80);
        assert_eq!(right.length, 20);
        assert_eq!(right.to_start, 48);
    }

    #[test]
    fn ch05_map_range_contains() {
        let line = "45 77 23";
        let map_range = MapRange::from(line);

        assert!(map_range.contains(77));
        assert!(map_range.contains(99));
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

    #[test]
    fn ch05_resource_map_normalize() {
        let lines_1 = vec![
            "seed-to-soil map:  ".into(),
            "50 98 2".into(),
            "52 50 48".into(),
        ];

        let lines_2 = vec![
            "soil-to-fertilizer map:  ".into(),
            "0 15 37".into(),
            "37 52 2".into(),
            "39 0 15".into(),
        ];

        let previous = ResourceMap::new(lines_1);
        let mut current = ResourceMap::new(lines_2);

        current.normalize(&previous);

        let mut expected = vec![
            MapRange {
                from_start: 0,
                to_start: 39,
                length: 15,
            },
            MapRange {
                from_start: 15,
                to_start: 0,
                length: 35,
            },
            MapRange {
                from_start: 50,
                to_start: 35,
                length: 2,
            },
            MapRange {
                from_start: 52,
                to_start: 37,
                length: 2,
            },
            MapRange {
                from_start: 54,
                to_start: 54,
                length: 46,
            },
        ];

        assert_eq!(current.ranges.sort(), expected.sort());
    }

    #[test]
    fn ch05_resource_map_merge() {
        let lines_1 = vec![
            "seed-to-soil map:  ".into(),
            "50 98 2".into(),
            "52 50 48".into(),
        ];

        let lines_2 = vec![
            "soil-to-fertilizer map:  ".into(),
            "0 15 37".into(),
            "37 52 2".into(),
            "39 0 15".into(),
        ];

        let previous = ResourceMap::new(lines_1);
        let mut current = ResourceMap::new(lines_2);

        current.merge_maps(&previous);

        let mut expected = vec![
            MapRange {
                from_start: 0,
                to_start: 39,
                length: 15,
            },
            MapRange {
                from_start: 15,
                to_start: 0,
                length: 35,
            },
            MapRange {
                from_start: 50,
                to_start: 37,
                length: 2,
            },
            MapRange {
                from_start: 52,
                to_start: 54,
                length: 46,
            },
            MapRange {
                from_start: 98,
                to_start: 35,
                length: 2,
            },
        ];

        assert_eq!(current.ranges.sort(), expected.sort());
    }
}
