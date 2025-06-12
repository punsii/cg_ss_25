use cg_ss_25::lib::{
    common::EPSILON,
    data::read_lines_from_file,
    event::{Event, EventHeap},
};
use ordered_float::NotNan;
use std::collections::BinaryHeap;
use std::f64::INFINITY;

fn get_next_event(
    start_events: &mut EventHeap,
    end_events: &mut EventHeap,
    vertical_events: &mut EventHeap,
) -> Option<Event> {
    let mut min_x = NotNan::new(INFINITY).unwrap();
    let mut min_index = -1;
    match start_events.peek() {
        Some(event) => {
            if event.x.0 < min_x {
                min_x = event.x.0;
                min_index = 0;
            }
        }
        None => {}
    }
    match end_events.peek() {
        Some(event) => {
            if event.x.0 < min_x {
                min_x = event.x.0;
                min_index = 1;
            }
        }
        None => {}
    }
    match vertical_events.peek() {
        Some(event) => {
            if event.x.0 < min_x {
                // min_x = event.x.0;
                min_index = 2;
            }
        }
        None => {}
    }

    match min_index {
        0 => Some(start_events.pop().unwrap()),
        1 => Some(end_events.pop().unwrap()),
        2 => Some(vertical_events.pop().unwrap()),
        _ => None,
    }
}

fn main() {
    let lines = read_lines_from_file("../data/01/s_1000_1.dat");

    // These are min heaps
    // => They are sorted while inserting and always pop the smallest element first
    let mut start_events: EventHeap = BinaryHeap::new();
    let mut end_events: EventHeap = BinaryHeap::new();
    let mut vertical_events: EventHeap = BinaryHeap::new();

    for line in &lines {
        let delta = (line.p1.x - line.p2.x).abs();

        if delta > EPSILON {
            start_events.push(Event::new(line.p1.x.min(line.p2.x), line.clone()));
            end_events.push(Event::new(line.p1.x.max(line.p2.x), line.clone()));
        } else {
            vertical_events.push(Event::new(line.p1.x, line.clone()));
        }
    }

    while let Some(event) = get_next_event(&mut start_events, &mut end_events, &mut vertical_events)
    {
        // Handle events here
        println!("{:?}", event.x.0);
    }
}
