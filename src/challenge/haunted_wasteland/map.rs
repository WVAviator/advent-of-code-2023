use std::collections::HashMap;

use super::node::Node;

#[derive(Default)]
pub struct Map {
    map: HashMap<String, Node>,
}

impl Map {
    pub fn new(lines: Vec<String>) -> Self {
        let map = lines
            .into_iter()
            .map(|line| {
                let node = Node::new(line);
                return (node.address.clone(), node);
            })
            .collect::<HashMap<String, Node>>();

        Map { map }
    }

    pub fn iter(&self) -> std::collections::hash_map::Iter<String, Node> {
        self.map.iter()
    }

    pub fn travel(&self, location: &str, direction: &char) -> &str {
        match direction {
            'L' | 'R' => self
                .map
                .get(location)
                .expect(format!("Got lost at unknown location: {}", location).as_str())
                .get_next(direction),
            _ => panic!("Cannot travel in the {} direction.", direction),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ch08_map_new() {
        let lines = vec![
            String::from("AAA = (BBB, BBB)"),
            String::from("BBB = (DDD, EEE)"),
        ];
        let map = Map::new(lines);

        assert!(map.map.get("AAA").is_some());
        assert!(map.map.get("BBB").is_some());
    }

    #[test]
    fn ch08_map_travel() {
        let lines = vec![
            String::from("AAA = (BBB, BBB)"),
            String::from("BBB = (AAA, ZZZ)"),
            String::from("ZZZ = (ZZZ, ZZZ)"),
        ];
        let map = Map::new(lines);

        assert_eq!(map.travel("AAA", &'L'), "BBB");
        assert_eq!(map.travel("BBB", &'L'), "AAA");
        assert_eq!(map.travel("AAA", &'R'), "BBB");
        assert_eq!(map.travel("BBB", &'R'), "ZZZ");
        assert_eq!(map.travel("ZZZ", &'L'), "ZZZ");
    }
}
