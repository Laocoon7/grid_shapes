use std::collections::HashSet;

use crate::{line::Line, Coord};

use super::line::BresenhamLineIter;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CircleIterStep {
    Line1,
    Line2,
    Line3,
    Line4,
    Process,
    Finished,
}

impl CircleIterStep {
    pub fn next(self) -> Self {
        match self {
            CircleIterStep::Line1 => CircleIterStep::Line2,
            CircleIterStep::Line2 => CircleIterStep::Line3,
            CircleIterStep::Line3 => CircleIterStep::Line4,
            CircleIterStep::Line4 => CircleIterStep::Process,
            CircleIterStep::Process => CircleIterStep::Line1,
            CircleIterStep::Finished => CircleIterStep::Finished,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CircleIter<C: Coord> {
    center: C,
    discovered: HashSet<C>,
    d: i32,
    x: i32,
    y: i32,
    step: CircleIterStep,
    line: Option<BresenhamLineIter<C>>,
}

impl<C: Coord> CircleIter<C> {
    pub fn new(center: C, radius: u32) -> Self {
        Self {
            center,
            discovered: HashSet::new(),
            d: (5 - (radius as i32 * 4)) / 4,
            x: 0,
            y: radius as i32,
            step: CircleIterStep::Line1,
            line: None,
        }
    }

    fn try_continue_line(&mut self) -> Option<C> {
        if let Some(line) = &mut self.line {
            // Does the line have a next point?
            if let Some(p) = line.next() {
                // Has this point been discovered?
                if self.discovered.insert(p) {
                    // It's new, return it
                    return Some(p);
                } else {
                    // Already seen this one, move on
                    return self.try_continue_line();
                }
            }
        }
        // No line / no more points
        None
    }

    fn try_next_line(&mut self) -> Option<BresenhamLineIter<C>> {
        let line = match self.step {
            // Pick Line1
            CircleIterStep::Line1 => Some(
                Line::new(
                    C::new(self.center.x() + self.x, self.center.y() + self.y),
                    C::new(self.center.x() + self.x, self.center.y() - self.y),
                )
                .into_iter(),
            ),
            // Pick Line2
            CircleIterStep::Line2 => Some(
                Line::new(
                    C::new(self.center.x() - self.x, self.center.y() + self.y),
                    C::new(self.center.x() - self.x, self.center.y() - self.y),
                )
                .into_iter(),
            ),
            // Pick Line3
            CircleIterStep::Line3 => Some(
                Line::new(
                    C::new(self.center.x() + self.y, self.center.y() + self.x),
                    C::new(self.center.x() + self.y, self.center.y() - self.x),
                )
                .into_iter(),
            ),
            // Pick Line4
            CircleIterStep::Line4 => Some(
                Line::new(
                    C::new(self.center.x() - self.y, self.center.y() + self.x),
                    C::new(self.center.x() - self.y, self.center.y() - self.x),
                )
                .into_iter(),
            ),
            // Process for next lines
            CircleIterStep::Process => {
                if self.d < 0 {
                    self.d += (2 * self.x) + 1;
                } else {
                    self.d += (2 * (self.x - self.y)) + 1;
                    self.y -= 1;
                }
                self.x += 1;

                // Check if we are finished
                if self.x > self.y {
                    self.step = CircleIterStep::Finished;
                    return None;
                }

                // Go back to line1
                self.step = self.step.next();
                // try again
                return self.try_next_line();
            }
            // We shouldn't hit this ever
            CircleIterStep::Finished => None,
        };

        // Move to the next Line
        self.step = self.step.next();
        // Return this line
        line
    }
}

impl<C: Coord> Iterator for CircleIter<C> {
    type Item = C;
    fn next(&mut self) -> Option<Self::Item> {
        // Are we done?
        if self.step == CircleIterStep::Finished {
            return None;
        }

        // Are we in the middle of a line?
        if let Some(c) = self.try_continue_line() {
            return Some(c);
        }

        // Get the next line
        if let Some(line) = self.try_next_line() {
            // Set it as the active line
            self.line = Some(line);
            // try again
            return self.next();
        }

        // couldn't get the next line
        None
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CircleCircumferenceIterStep {
    Line1,
    Line2,
    Line3,
    Line4,
    Line5,
    Line6,
    Line7,
    Line8,
    Process,
    Finished,
}

impl CircleCircumferenceIterStep {
    pub fn next(self) -> Self {
        match self {
            CircleCircumferenceIterStep::Line1 => CircleCircumferenceIterStep::Line2,
            CircleCircumferenceIterStep::Line2 => CircleCircumferenceIterStep::Line3,
            CircleCircumferenceIterStep::Line3 => CircleCircumferenceIterStep::Line4,
            CircleCircumferenceIterStep::Line4 => CircleCircumferenceIterStep::Line5,
            CircleCircumferenceIterStep::Line5 => CircleCircumferenceIterStep::Line6,
            CircleCircumferenceIterStep::Line6 => CircleCircumferenceIterStep::Line7,
            CircleCircumferenceIterStep::Line7 => CircleCircumferenceIterStep::Line8,
            CircleCircumferenceIterStep::Line8 => CircleCircumferenceIterStep::Process,
            CircleCircumferenceIterStep::Process => CircleCircumferenceIterStep::Line1,
            CircleCircumferenceIterStep::Finished => CircleCircumferenceIterStep::Finished,
        }
    }
}

#[derive(Debug, Clone)]
pub struct CircleCircumferenceIter<C: Coord> {
    center: C,
    discovered: HashSet<C>,
    d: i32,
    x: i32,
    y: i32,
    step: CircleCircumferenceIterStep,
}

impl<C: Coord> CircleCircumferenceIter<C> {
    pub fn new(center: C, radius: u32) -> Self {
        Self {
            center,
            discovered: HashSet::new(),
            d: (5 - (radius as i32 * 4)) / 4,
            x: 0,
            y: radius as i32,
            step: CircleCircumferenceIterStep::Line1,
        }
    }

    fn try_next_point(&mut self) -> Option<C> {
        let c = match self.step {
            CircleCircumferenceIterStep::Line1 => {
                Some(C::new(self.center.x() + self.x, self.center.y() + self.y))
            }
            CircleCircumferenceIterStep::Line2 => {
                Some(C::new(self.center.x() + self.x, self.center.y() - self.y))
            }
            CircleCircumferenceIterStep::Line3 => {
                Some(C::new(self.center.x() - self.x, self.center.y() + self.y))
            }
            CircleCircumferenceIterStep::Line4 => {
                Some(C::new(self.center.x() - self.x, self.center.y() - self.y))
            }
            CircleCircumferenceIterStep::Line5 => {
                Some(C::new(self.center.x() + self.y, self.center.y() + self.x))
            }
            CircleCircumferenceIterStep::Line6 => {
                Some(C::new(self.center.x() + self.y, self.center.y() - self.x))
            }
            CircleCircumferenceIterStep::Line7 => {
                Some(C::new(self.center.x() - self.y, self.center.y() + self.x))
            }
            CircleCircumferenceIterStep::Line8 => {
                Some(C::new(self.center.x() - self.y, self.center.y() - self.x))
            }
            CircleCircumferenceIterStep::Process => {
                if self.d < 0 {
                    self.d += (2 * self.x) + 1;
                } else {
                    self.d += (2 * (self.x - self.y)) + 1;
                    self.y -= 1;
                }

                self.x += 1;

                if self.x > self.y {
                    self.step = CircleCircumferenceIterStep::Finished;
                    return None;
                }

                self.step = self.step.next();
                return self.try_next_point();
            }
            CircleCircumferenceIterStep::Finished => None,
        };

        self.step = self.step.next();

        c
    }
}

impl<C: Coord> Iterator for CircleCircumferenceIter<C> {
    type Item = C;
    fn next(&mut self) -> Option<Self::Item> {
        // Are we done?
        if self.step == CircleCircumferenceIterStep::Finished {
            return None;
        }

        // Get the next point
        if let Some(c) = self.try_next_point() {
            // Make sure it's unique
            if self.discovered.insert(c) {
                // It's new return it
                return Some(c);
            } else {
                // It's old, try the next one
                return self.next();
            }
        }

        // Couldn't get the next point
        None
    }
}
