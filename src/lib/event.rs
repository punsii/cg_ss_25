use crate::lib::line::Line;

// Derivative provides 'Derive' makros that can ignore parameters.
// => Events should be comparable only by their x-value, not by the referenced 'Line'
use derivative::Derivative;

// Helper float class that has a total order for sorting
// (f64 only has a partial order which causes it to not be sortable by deafault)
use ordered_float::NotNan;

// Using cmp::Reverse causes the BinaryHeap to be a minHeap instead of a maxHeap
use std::{cmp::Reverse, collections::BinaryHeap, fmt};
use crate::lib::point::Point;

#[derive(Debug, Clone, PartialEq)]
pub enum EventType {
    Start,
    End,
    Intersection(Point, Line, Line),
}

pub type EventHeap = BinaryHeap<Event>;

#[derive(Derivative)]
#[derivative(Eq, PartialEq, Ord, PartialOrd)]
pub struct Event {
    // Events are sortable by their x value, with the smallest x value being the first element.
    pub x: Reverse<NotNan<f64>>,
    #[derivative(PartialEq = "ignore", Ord = "ignore", PartialOrd = "ignore")]
    pub line: Line,
    #[derivative(PartialEq = "ignore", Ord = "ignore", PartialOrd = "ignore")]
    pub event_type: EventType,
}

impl fmt::Debug for Event {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Event ({})", self.x.0)
    }
}

impl Event {
    // Helper for constructing an event withouth having to deal with NotNan / Reverse classes
    pub fn new(x: f64, line: Line, event_type: EventType) -> Self {
        Self {
            x: Reverse(NotNan::new(x).unwrap()),
            line: line,
            event_type: event_type,
        }
    }
}
