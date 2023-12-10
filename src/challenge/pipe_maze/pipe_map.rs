use std::collections::HashSet;

use super::{pipe::Pipe, pipe_type::PipeType};

#[derive(Default)]
pub struct PipeMap {
    map: Vec<Vec<Pipe>>,
    start: (usize, usize),
}

impl PipeMap {
    pub fn new(lines: Vec<String>) -> Self {
        let mut start: Option<(usize, usize)> = None;
        let map = lines
            .into_iter()
            .enumerate()
            .map(|(line_index, line)| {
                line.chars()
                    .enumerate()
                    .map(|(c_index, c)| {
                        let pipe = Pipe::new(c, (line_index, c_index));
                        if pipe.pipe_type == PipeType::Start {
                            start = Some((line_index, c_index));
                        }
                        pipe
                    })
                    .collect::<Vec<Pipe>>()
            })
            .collect::<Vec<Vec<Pipe>>>();

        let start = start.expect("No start position found in pipe map.");

        PipeMap { map, start }
    }

    pub fn get_connected(&self, pipe: &Pipe) -> Vec<&Pipe> {
        let (row, col) = pipe.position;
        let mut pipe_type = pipe.pipe_type;

        if let PipeType::Start = pipe_type {
            let up: Option<PipeType> = {
                if row != 0 {
                    Some(self.map[row - 1][col].pipe_type)
                } else {
                    None
                }
            };
            let right: Option<PipeType> = {
                if col < self.map[0].len() - 1 {
                    Some(self.map[row][col + 1].pipe_type)
                } else {
                    None
                }
            };
            let down: Option<PipeType> = {
                if row < self.map.len() - 1 {
                    Some(self.map[row + 1][col].pipe_type)
                } else {
                    None
                }
            };
            let left: Option<PipeType> = {
                if col != 0 {
                    Some(self.map[row][col - 1].pipe_type)
                } else {
                    None
                }
            };

            pipe_type = match (up, right, down, left) {
                (
                    Some(PipeType::ElbowSW | PipeType::ElbowSE | PipeType::Vertical),
                    _,
                    Some(PipeType::ElbowNE | PipeType::ElbowNW | PipeType::Vertical),
                    _,
                ) => PipeType::Vertical,
                (
                    Some(PipeType::ElbowSW | PipeType::ElbowSE | PipeType::Vertical),
                    Some(PipeType::Horizontal | PipeType::ElbowNW | PipeType::ElbowSW),
                    _,
                    _,
                ) => PipeType::ElbowNE,
                (
                    Some(PipeType::ElbowSW | PipeType::ElbowSE | PipeType::Vertical),
                    _,
                    _,
                    Some(PipeType::Horizontal | PipeType::ElbowNE | PipeType::ElbowSE),
                ) => PipeType::ElbowNW,
                (
                    _,
                    Some(PipeType::Horizontal | PipeType::ElbowNW | PipeType::ElbowSW),
                    _,
                    Some(PipeType::Horizontal | PipeType::ElbowNE | PipeType::ElbowSE),
                ) => PipeType::Horizontal,
                (
                    _,
                    Some(PipeType::Horizontal | PipeType::ElbowNW | PipeType::ElbowSW),
                    Some(PipeType::ElbowNE | PipeType::ElbowNW | PipeType::Vertical),
                    _,
                ) => PipeType::ElbowSE,
                (
                    _,
                    _,
                    Some(PipeType::ElbowNE | PipeType::ElbowNW | PipeType::Vertical),
                    Some(PipeType::Horizontal | PipeType::ElbowNE | PipeType::ElbowSE),
                ) => PipeType::ElbowSW,
                (_, _, _, _) => panic!("Could not determine start position pipe type."),
            }
        }

        match pipe_type {
            PipeType::Vertical => {
                if row == 0 || row >= self.map.len() - 1 {
                    panic!("Pipe network hit vertical edge of map at row {}.", row);
                }
                vec![&self.map[row - 1][col], &self.map[row + 1][col]]
            }
            PipeType::Horizontal => {
                if col == 0 || col >= self.map[0].len() - 1 {
                    panic!("Pipe network hit horizontal edge of map at column {}.", col);
                }
                vec![&self.map[row][col - 1], &self.map[row][col + 1]]
            }
            PipeType::ElbowNE => {
                if row == 0 || col >= self.map[0].len() - 1 {
                    panic!(
                        "Pipe network hit edge of map at position ({}, {})",
                        row, col
                    );
                }
                vec![&self.map[row - 1][col], &self.map[row][col + 1]]
            }
            PipeType::ElbowNW => {
                if row == 0 || col == 0 {
                    panic!(
                        "Pipe network hit edge of map at position ({}, {})",
                        row, col
                    );
                }
                vec![&self.map[row - 1][col], &self.map[row][col - 1]]
            }
            PipeType::ElbowSW => {
                if row >= self.map.len() || col == 0 {
                    panic!(
                        "Pipe network hit edge of map at position ({}, {})",
                        row, col
                    );
                }
                vec![&self.map[row + 1][col], &self.map[row][col - 1]]
            }
            PipeType::ElbowSE => {
                if row >= self.map.len() - 1 || col >= self.map[0].len() - 1 {
                    panic!(
                        "Pipe network hit edge of map at position ({}, {})",
                        row, col
                    );
                }
                vec![&self.map[row + 1][col], &self.map[row][col + 1]]
            }
            PipeType::Start => panic!("Start pipe not converted."),
            PipeType::Ground => panic!("Can't navigate pipe network from ground tile."),
        }
    }

    pub fn at(&self, row: usize, col: usize) -> &Pipe {
        &self.map[row][col]
    }
}

impl<'a> IntoIterator for &'a PipeMap {
    type Item = &'a Pipe;

    type IntoIter = PipeNetworkIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        PipeNetworkIterator::new(self)
    }
}

pub struct PipeNetworkIterator<'a> {
    current: Option<(usize, usize)>,
    pipe_map: &'a PipeMap,
    visited: HashSet<(usize, usize)>,
}

impl<'a> PipeNetworkIterator<'a> {
    pub fn new(pipe_map: &'a PipeMap) -> Self {
        let current = Some(pipe_map.start);
        let visited = HashSet::new();
        PipeNetworkIterator {
            current,
            pipe_map,
            visited,
        }
    }
}
impl<'a> Iterator for PipeNetworkIterator<'a> {
    type Item = &'a Pipe;

    fn next(&mut self) -> Option<Self::Item> {
        if let None = self.current {
            return None;
        }
        let current = self.current.unwrap();
        let (cur_row, cur_col) = current;
        self.visited.insert((cur_row, cur_col));
        let cur_pipe = self.pipe_map.at(cur_row, cur_col);
        let mut connected = self.pipe_map.get_connected(cur_pipe);
        connected.sort();
        let next_pipe = connected
            .into_iter()
            .skip_while(|pipe| self.visited.contains(&pipe.position))
            .next();
        self.current = match next_pipe {
            Some(pipe) => Some(pipe.position),
            None => None,
        };

        return Some(cur_pipe);
    }
}

impl<'a> DoubleEndedIterator for PipeNetworkIterator<'a> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if let None = self.current {
            return None;
        }
        let current = self.current.unwrap();
        let (cur_row, cur_col) = current;
        self.visited.insert((cur_row, cur_col));
        let cur_pipe = self.pipe_map.at(cur_row, cur_col);
        let mut connected = self.pipe_map.get_connected(cur_pipe);
        connected.sort_by(|a, b| b.cmp(a));
        let next_pipe = connected
            .into_iter()
            .skip_while(|pipe| self.visited.contains(&pipe.position))
            .next();
        self.current = match next_pipe {
            Some(pipe) => Some(pipe.position),
            None => None,
        };

        return Some(cur_pipe);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn ch10_pipe_map_new() {
        let lines = vec![String::from("S7"), String::from("LJ")];

        let pipe_map = PipeMap::new(lines);

        let expected = vec![
            vec![Pipe::new('S', (0, 0)), Pipe::new('7', (0, 1))],
            vec![Pipe::new('L', (1, 0)), Pipe::new('J', (1, 1))],
        ];

        assert_eq!(pipe_map.map, expected);
    }

    #[test]
    fn ch10_pipe_map_get_connected() {
        let lines = vec![String::from("S7"), String::from("LJ")];
        let pipe_map = PipeMap::new(lines);

        let pipe_ref = &pipe_map.map[1][1];

        let mut connected = pipe_map.get_connected(pipe_ref);
        let mut expected = vec![&pipe_map.map[1][0], &pipe_map.map[0][1]];

        connected.sort();
        expected.sort();

        assert_eq!(connected, expected);
    }

    #[test]
    fn ch10_pipe_map_iterator() {
        let lines = vec![String::from("S7"), String::from("LJ")];
        let pipe_map = PipeMap::new(lines);
        let mut pipe_iter = pipe_map.into_iter();

        assert_eq!(pipe_iter.next().unwrap(), &pipe_map.map[0][0]);
        assert_eq!(pipe_iter.next().unwrap(), &pipe_map.map[0][1]);
        assert_eq!(pipe_iter.next().unwrap(), &pipe_map.map[1][1]);
        assert_eq!(pipe_iter.next().unwrap(), &pipe_map.map[1][0]);
        assert!(pipe_iter.next().is_none());
    }

    #[test]
    fn ch10_pipe_map_iterator_rev() {
        let lines = vec![String::from("S7"), String::from("LJ")];
        let pipe_map = PipeMap::new(lines);
        let mut pipe_iter = pipe_map.into_iter().rev();

        assert_eq!(pipe_iter.next().unwrap(), &pipe_map.map[0][0]);
        assert_eq!(pipe_iter.next().unwrap(), &pipe_map.map[1][0]);
        assert_eq!(pipe_iter.next().unwrap(), &pipe_map.map[1][1]);
        assert_eq!(pipe_iter.next().unwrap(), &pipe_map.map[0][1]);
        assert!(pipe_iter.next().is_none());
    }
}
