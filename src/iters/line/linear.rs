#[derive(Debug, Clone)]
pub struct LinearIter {
    current: i32,
    max: i32,
}

impl LinearIter {
    pub fn new(start: i32, end: i32) -> Self {
        Self {
            current: start.min(end),
            max: start.max(end),
        }
    }
}

impl Iterator for LinearIter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > self.max {
            return None;
        }

        let v = self.current;
        self.current += 1;

        Some(v)
    }
}
