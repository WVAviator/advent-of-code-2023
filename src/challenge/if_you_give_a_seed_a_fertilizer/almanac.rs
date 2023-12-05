use std::collections::HashMap;

use super::{resource::Resource, resource_map::ResourceMap};

#[derive(Default)]
pub struct Almanac {
    maps: HashMap<Resource, ResourceMap>,
}

impl Almanac {
    pub fn new(lines: Vec<String>) -> Self {
        let mut maps = HashMap::new();

        lines.split(|el| el.is_empty()).for_each(|group| {
            let resource_map = ResourceMap::new(group.into());
            maps.insert(resource_map.get_from(), resource_map);
        });

        Almanac { maps }
    }

    pub fn condense(&self) -> ResourceMap {
        let mut maps = self.maps.clone();
        let mut current = maps.remove(&Resource::Seed).expect("No seed in map.");
        while let Some(mut next) = maps.remove(&current.get_to()) {
            next.merge_maps(&current);
            current = next;
        }

        current
    }

    pub fn map_through(&self, seed: u64, from: Resource) -> (u64, Resource) {
        let mut current_resource = from;
        let mut current_value = seed;
        while let Some(resource_map) = self.maps.get(&current_resource) {
            (current_value, current_resource) = resource_map.map_to(current_value);
        }

        (current_value, current_resource)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn create_test_case() -> Almanac {
        let lines = vec![
            String::from("seed-to-soil map:"),
            String::from("50 98 2"),
            String::from("52 50 48"),
            String::from(""),
            String::from("soil-to-fertilizer map:"),
            String::from("0 15 37"),
            String::from("37 52 2"),
            String::from("37 52 2"),
        ];

        Almanac::new(lines)
    }

    #[test]
    fn ch05_almanac_new() {
        let almanac = create_test_case();

        assert_eq!(almanac.maps.len(), 2);
        assert!(almanac.maps.get(&Resource::Seed).is_some());
        assert!(almanac.maps.get(&Resource::Soil).is_some());
    }

    #[test]
    fn ch05_almanac_map_through() {
        let almanac = create_test_case();

        let (value, resource) = almanac.map_through(79, Resource::Seed);
        assert_eq!(value, 81);
        assert_eq!(resource, Resource::Fertilizer);
    }
}
