use crate::line::Line;

#[derive(PartialEq, Debug, Default)]
pub struct Entry {
    pub lines: Vec<Line>,
}

impl Entry {
    pub fn new() -> Entry {
        Entry { lines: Vec::new() }
    }

    pub fn add_line(&mut self, line: Line) {
        self.lines.push(line);
    }

    pub fn get_lines(&self) -> &Vec<Line> {
        &self.lines
    }
}
