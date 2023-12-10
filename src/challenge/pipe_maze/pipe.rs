use super::pipe_type::PipeType;

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct Pipe {
    pub position: (usize, usize),
    pub pipe_type: PipeType,
}

impl Pipe {
    pub fn new(c: char, position: (usize, usize)) -> Self {
        let pipe_type = PipeType::from(c);
        Pipe {
            position,
            pipe_type,
        }
    }
}
