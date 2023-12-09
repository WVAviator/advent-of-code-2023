#[derive(Debug, PartialEq)]
pub struct Node {
    pub address: String,
    pub left_address: String,
    pub right_address: String,
}

impl Node {
    pub fn new(line: String) -> Self {
        // AAA = (BBB, BBB)
        let address = line[..3].to_string();
        let left_address = line[7..10].to_string();
        let right_address = line[12..15].to_string();

        Node {
            address,
            left_address,
            right_address,
        }
    }

    pub fn get_next(&self, dir: &char) -> &String {
        match dir {
            'L' => &self.left_address,
            'R' => &self.right_address,
            _ => panic!("Invalid character in directions."),
        }
    }

    pub fn matches(&self, pattern: &str) -> bool {
        pattern
            .chars()
            .zip(self.address.chars())
            .all(|(a, b)| a == '_' || a == b)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ch08_node_new() {
        let node = Node::new(String::from("AAA = (BBB, CCC)"));
        let expected = Node {
            address: String::from("AAA"),
            left_address: String::from("BBB"),
            right_address: String::from("CCC"),
        };
        assert_eq!(node, expected);
    }

    #[test]
    fn ch08_node_get_next() {
        let node = Node::new(String::from("AAA = (BBB, CCC)"));
        assert_eq!(node.get_next(&'L'), "BBB");
        assert_eq!(node.get_next(&'R'), "CCC");
    }

    #[test]
    fn ch08_map_matches() {
        let node = Node::new(String::from("ABC = (BBB, BBB)"));

        assert!(node.matches("ABC"));
        assert!(node.matches("A_C"));
        assert!(node.matches("__C"));
        assert!(node.matches("___"));
        assert!(!node.matches("ABD"));
        assert!(!node.matches("AD_"));
        assert!(!node.matches("__Z"));
        assert!(!node.matches("_BF"));
    }
}
