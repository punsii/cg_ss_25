use cg_ss_25::lib::ccw::crossing_point;
use cg_ss_25::lib::event::EventType;
use cg_ss_25::lib::event::EventType::{End, Intersection, Start};
use cg_ss_25::lib::line_with_ord::LineWithOrd;
use cg_ss_25::lib::{
    common::EPSILON,
    data::read_lines_from_file,
    event::{Event, EventHeap},
};
use std::collections::{BTreeSet, BinaryHeap};
use std::time::Instant;

fn treat_left_endpoint(
    event: &Event,
    sweep_line: &mut BTreeSet<LineWithOrd>,
    event_queue: &mut BinaryHeap<Event>,
    current_x: f64,
) {
    let line = event.line.clone();
    let current_seg = LineWithOrd::new(line.clone(), current_x);

    sweep_line.insert(current_seg.clone());

    let above = sweep_line.range(current_seg.clone()..).nth(1);
    let below = sweep_line.range(..current_seg.clone()).next_back();

    if let Some(above_seg) = above {
        if line.crosses(&above_seg.line) {
            let p = crossing_point(&line.p1, &line.p2, &above_seg.line.p1, &above_seg.line.p2);
            if p.x > current_x {
                event_queue.push(Event::new(
                    p.x,
                    line.clone(),
                    Intersection(p, line.clone(), above_seg.line.clone()),
                ));
            }
        }
    }

    if let Some(below_seg) = below {
        if line.crosses(&below_seg.line) {
            let p = crossing_point(&line.p1, &line.p2, &below_seg.line.p1, &below_seg.line.p2);
            if p.x > current_x {
                event_queue.push(Event::new(
                    p.x,
                    line.clone(),
                    Intersection(p, line.clone(), below_seg.line.clone()),
                ));
            }
        }
    }
}

fn treat_right_endpoint(
    event: &Event,
    sweep_line: &mut BTreeSet<LineWithOrd>,
    event_queue: &mut BinaryHeap<Event>,
    current_x: f64,
) {
    let line = event.line.clone();
    let seg = LineWithOrd::new(line.clone(), current_x);

    let above = sweep_line.range(seg.clone()..).nth(1).cloned();
    let below = sweep_line.range(..seg.clone()).next_back().cloned();

    sweep_line.remove(&seg);

    if let (Some(above_seg), Some(below_seg)) = (above, below) {
        if above_seg.line.crosses(&below_seg.line) {
            let p = crossing_point(
                &above_seg.line.p1,
                &above_seg.line.p2,
                &below_seg.line.p1,
                &below_seg.line.p2,
            );
            if p.x > current_x {
                event_queue.push(Event::new(
                    p.x,
                    above_seg.line.clone(),
                    EventType::Intersection(p, line.clone(), above_seg.line.clone()),
                ));
            }
        }
    }
}

fn treat_intersection(
    event: &Event,
    sweep_line: &mut BTreeSet<LineWithOrd>,
    event_queue: &mut BinaryHeap<Event>,
    current_x: f64,
) {
    let current_line = event.line.clone();
    let current_seg = LineWithOrd::new(current_line.clone(), current_x);

    let above = sweep_line.range(current_seg.clone()..).nth(1).cloned();
    let below = sweep_line.range(..current_seg.clone()).next_back().cloned();

    sweep_line.remove(&current_seg);
    if let Some(ref above_seg) = above {
        sweep_line.remove(above_seg);
    }

    if let Some(above_seg) = &above {
        let mut seg1 = current_seg.clone();
        seg1.sweep_x = current_x + EPSILON;

        let mut seg2 = above_seg.clone().clone();
        seg2.sweep_x = current_x + EPSILON;

        sweep_line.insert(seg2.clone());
        sweep_line.insert(seg1.clone());

        let new_above = sweep_line.range(seg2.clone()..).nth(1);
        if let Some(a) = new_above {
            if seg2.line.crosses(&a.line) {
                let p = crossing_point(&seg2.line.p1, &seg2.line.p2, &a.line.p1, &a.line.p2);
                if p.x > current_x {
                    event_queue.push(Event::new(
                        p.x,
                        seg2.line.clone(),
                        EventType::Intersection(p, seg2.line.clone(), seg1.line.clone()),
                    ));
                }
            }
        }

        let new_below = sweep_line.range(..seg1.clone()).next_back();
        if let Some(b) = new_below {
            if seg1.line.crosses(&b.line) {
                let p = crossing_point(&seg1.line.p1, &seg1.line.p2, &b.line.p1, &b.line.p2);
                if p.x > current_x {
                    event_queue.push(Event::new(
                        p.x,
                        seg1.line.clone(),
                        EventType::Intersection(p, seg1.line.clone(), seg2.line.clone()),
                    ));
                }
            }
        }
    }
}

fn main() {
    let mut files = Vec::new();
    files.push("../data/01/s_1000_1.dat");
    files.push("../data/01/s_10000_1.dat");
    files.push("../data/01/s_100000_1.dat");
    files.push("../data/01/s_1000_10.dat");

    for file in files {
        let mut intersection_count = 0;
        println!(
            "{}",
            file.split("/").last().unwrap().split(".").next().unwrap()
        );
        let lines = read_lines_from_file(file);

        // The BinaryHeap is a min heap
        // => It is sorted while inserting and always pops the smallest element first
        let mut event_queue: EventHeap = BinaryHeap::new();
        let mut sweep_line: BTreeSet<LineWithOrd> = BTreeSet::new();

        let mut number_of_segments = 0;
        for line in &lines {
            if (line.p1.x - line.p2.x).abs() < EPSILON {
                continue;
            }
            number_of_segments += 1;
            let (left, right) = if line.p1.x < line.p2.x {
                (&line.p1, &line.p2)
            } else {
                (&line.p2, &line.p1)
            };
            event_queue.push(Event::new(left.x, line.clone(), Start));
            event_queue.push(Event::new(right.x, line.clone(), End));
        }

        let start_time = Instant::now();
        while let Some(event) = event_queue.pop() {
            let current_x = event.x.0.into_inner();
            match &event.event_type {
                Start => treat_left_endpoint(&event, &mut sweep_line, &mut event_queue, current_x),
                End => treat_right_endpoint(&event, &mut sweep_line, &mut event_queue, current_x),
                Intersection(p, l1, l2) => {
                    intersection_count += 1;
                    treat_intersection(&event, &mut sweep_line, &mut event_queue, current_x)
                }
            }
        }
        let duration = start_time.elapsed();

        println!("intersection count is {}", intersection_count);
        println!("time: {:?}", duration);
        println!("number of segments: {}", number_of_segments);
    }
}
