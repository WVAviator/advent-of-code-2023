#[derive(Debug, Eq, PartialEq, Ord, PartialOrd, Clone, Copy)]
pub enum PipeType {
    Vertical,
    Horizontal,
    ElbowNE,
    ElbowNW,
    ElbowSW,
    ElbowSE,
    Start,
    Ground,
}

impl PipeType {
    pub fn from(c: char) -> PipeType {
        match c {
            '|' => PipeType::Vertical,
            '-' => PipeType::Horizontal,
            'L' => PipeType::ElbowNE,
            'J' => PipeType::ElbowNW,
            '7' => PipeType::ElbowSW,
            'F' => PipeType::ElbowSE,
            '.' => PipeType::Ground,
            'S' => PipeType::Start,
            _ => panic!("Invalid pipe type {}", c),
        }
    }
}
