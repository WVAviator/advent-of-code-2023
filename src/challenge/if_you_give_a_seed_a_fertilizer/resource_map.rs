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

    pub fn map_to(&self, from: num::BigInt) -> (num::BigInt, Resource) {
        let mut to = from.clone();
        for range in &self.ranges {
            if range.contains(&from) {
                to = range.map_to(&from);
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
                if prev_range.contains_to(&range.from_start) {
                    range.add(&prev_range.get_diff());
                }
            }
        })
    }

    fn normalize(&mut self, previous: &ResourceMap) {
        let mut split_values: Vec<num::BigInt> = previous
            .ranges
            .iter()
            .flat_map(|range| [range.from_start.clone(), range.from_end()].into_iter())
            .collect();
        split_values.sort();
        let new_ranges = self
            .ranges
            .iter()
            .flat_map(|range| {
                let mut parts = Vec::new();
                let mut right = range.clone();
                let mut i = 0;
                while i < split_values.len() {
                    if let Some((l, r)) = right.split_on(&split_values[i]) {
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

    pub fn lowest_overlap(&self, start: &num::BigInt, length: &num::BigInt) -> num::BigInt {
        let map_range = MapRange {
            from_start: start.clone(),
            to_start: start.clone(),
            length: length.clone(),
        };
        let split_values: Vec<num::BigInt> = self
            .ranges
            .iter()
            .flat_map(|range| [range.from_start.clone(), range.from_end()].into_iter())
            .collect();
        [map_range]
            .iter()
            .flat_map(|range| {
                let mut parts = Vec::new();
                let mut right = range.clone();
                let mut i = 0;
                while i < split_values.len() {
                    if let Some((l, r)) = right.split_on(&split_values[i]) {
                        right = r;
                        parts.push(l)
                    }

                    i += 1;
                }
                parts.push(right);
                parts.into_iter()
            })
            .filter(|range| &range.from_start >= start && range.from_end() <= start + length)
            .map(|range| self.map_to(range.from_start).0)
            .min()
            .expect("No values to calculate minimum in map range.")
    }
}

#[derive(PartialEq, Debug, Clone, PartialOrd, Ord, Eq)]
pub struct MapRange {
    from_start: num::BigInt,
    to_start: num::BigInt,
    length: num::BigInt,
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

    pub fn contains(&self, value: &num::BigInt) -> bool {
        let range = &self.from_start..&(&self.from_start + &self.length);
        range.contains(&value)
    }

    pub fn contains_to(&self, value: &num::BigInt) -> bool {
        let range = &self.to_start..&(&self.to_start + &self.length);
        range.contains(&value)
    }

    pub fn get_diff(&self) -> num::BigInt {
        &self.to_start - &self.from_start
    }

    pub fn map_to(&self, from: &num::BigInt) -> num::BigInt {
        if !self.contains(from) {
            panic!("Attempted to map a value that is not in the range.");
        }
        let from_diff = from - &self.from_start;
        &self.to_start + from_diff
    }

    pub fn can_split(&self, value: &num::BigInt) -> bool {
        self.contains(value)
            && value != &self.from_start
            && value != &(&self.from_start + &self.length)
    }

    pub fn split_on(&self, value: &num::BigInt) -> Option<(Self, Self)> {
        if !self.can_split(value) {
            return None;
        }

        let left = MapRange {
            from_start: self.from_start.clone(),
            to_start: self.to_start.clone(),
            length: value - self.from_start.clone(),
        };
        let right = MapRange {
            from_start: value.clone(),
            to_start: self.to_start.clone() + (value - self.from_start.clone()),
            length: (self.from_start.clone() + self.length.clone()) - value,
        };

        Some((left, right))
    }

    pub fn from_end(&self) -> num::BigInt {
        &self.from_start + &self.length
    }

    pub fn add(&mut self, value: &num::BigInt) {
        let start = &self.from_start + value;
        self.from_start = start.clone();
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ch05_map_range_from() {
        let line = "45 77 23";
        let map_range = MapRange::from(line);

        assert_eq!(map_range.from_start, num::BigInt::from(77));
        assert_eq!(map_range.to_start, num::BigInt::from(45));
        assert_eq!(map_range.length, num::BigInt::from(23));
    }

    #[test]
    fn ch05_map_range_split_on() {
        let line = "45 77 23";
        let map_range = MapRange::from(line);
        let (left, right) = map_range.split_on(&num::BigInt::from(80)).unwrap();

        assert_eq!(left.from_start, num::BigInt::from(77));
        assert_eq!(left.length, num::BigInt::from(3));
        assert_eq!(left.to_start, num::BigInt::from(45));
        assert_eq!(right.from_start, num::BigInt::from(80));
        assert_eq!(right.length, num::BigInt::from(20));
        assert_eq!(right.to_start, num::BigInt::from(48));
    }

    #[test]
    fn ch05_map_range_contains() {
        let line = "45 77 23";
        let map_range = MapRange::from(line);

        assert!(map_range.contains(&num::BigInt::from(77)));
        assert!(map_range.contains(&num::BigInt::from(99)));
        assert!(map_range.contains(&num::BigInt::from(87)));
        assert!(!map_range.contains(&num::BigInt::from(76)));
        assert!(!map_range.contains(&num::BigInt::from(101)));
    }

    #[test]
    fn ch05_map_range_map_to() {
        let line = "45 77 23";
        let map_range = MapRange::from(line);

        assert_eq!(
            map_range.map_to(&num::BigInt::from(78)),
            num::BigInt::from(46)
        );
    }

    #[test]
    fn ch05_map_range_from_end_huge_numbers() {
        let line = "45 18446744073709551615 5";
        let map_range = MapRange::from(line);

        assert_eq!(
            map_range.from_end(),
            "18446744073709551620".parse::<num::BigInt>().unwrap()
        );
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
                from_start: num::BigInt::from(0),
                to_start: num::BigInt::from(39),
                length: num::BigInt::from(15),
            },
            MapRange {
                from_start: num::BigInt::from(15),
                to_start: num::BigInt::from(0),
                length: num::BigInt::from(35),
            },
            MapRange {
                from_start: num::BigInt::from(50),
                to_start: num::BigInt::from(35),
                length: num::BigInt::from(2),
            },
            MapRange {
                from_start: num::BigInt::from(52),
                to_start: num::BigInt::from(37),
                length: num::BigInt::from(2),
            },
            MapRange {
                from_start: num::BigInt::from(54),
                to_start: num::BigInt::from(54),
                length: num::BigInt::from(46),
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
                from_start: num::BigInt::from(0),
                to_start: num::BigInt::from(39),
                length: num::BigInt::from(15),
            },
            MapRange {
                from_start: num::BigInt::from(15),
                to_start: num::BigInt::from(0),
                length: num::BigInt::from(35),
            },
            MapRange {
                from_start: num::BigInt::from(50),
                to_start: num::BigInt::from(37),
                length: num::BigInt::from(2),
            },
            MapRange {
                from_start: num::BigInt::from(52),
                to_start: num::BigInt::from(54),
                length: num::BigInt::from(46),
            },
            MapRange {
                from_start: num::BigInt::from(98),
                to_start: num::BigInt::from(35),
                length: num::BigInt::from(2),
            },
        ];

        assert_eq!(current.ranges.sort(), expected.sort());
    }
}
