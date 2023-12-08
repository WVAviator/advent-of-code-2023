use std::collections::HashMap;

use super::node::Node;

#[derive(Default)]
pub struct Map {
    map: HashMap<String, Node>,
    pub current_location: String,
    pub steps: u32,
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

        let current_location = String::from("AAA");

        Map {
            map,
            current_location,
            steps: 0,
        }
    }

    pub fn travel(&mut self, direction: &char) -> &str {
        match direction {
            'L' | 'R' => {
                let next_address = self
                    .map
                    .get(&self.current_location)
                    .expect(
                        format!("Got lost at unknown location: {}", &self.current_location)
                            .as_str(),
                    )
                    .get_next(direction);
                self.current_location = next_address.clone();
                self.steps += 1;

                &self.current_location
            }
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
        let mut map = Map::new(lines);

        assert_eq!(map.travel(&'L'), "BBB");
        assert_eq!(map.travel(&'L'), "AAA");
        assert_eq!(map.travel(&'R'), "BBB");
        assert_eq!(map.travel(&'R'), "ZZZ");
        assert_eq!(map.travel(&'L'), "ZZZ");
        assert_eq!(map.steps, 5);
    }
}
