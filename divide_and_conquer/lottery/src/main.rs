use std::io;
use std::process;

fn main() {
    let (segments_number, points_number) = read_segments_points_number();
    let mut values: Vec<Value> = Vec::with_capacity(2*segments_number + points_number);

    read_segments(&mut values, segments_number);
    read_points(&mut values, points_number);

    let mut points = run_lottery(&mut values);
    points.sort_by(|a, b| a.order().cmp(&b.order()));

    let result = points.iter().map(|p| p.segments()).collect::<Vec<isize>>();

    let mut res = String::new();

    for p in result.iter() {
        if res.len() > 0 {
            res.push_str(" ");
        }

        res.push_str(&p.to_string());
    }

    println!("{}", res);
}

fn read_segments_points_number() -> (usize, usize) {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("can't read number");

    let mut slice: Vec<usize> = Vec::with_capacity(2);

    for val in input.trim().split_whitespace() {
        let value = val.parse().unwrap();
        if value < 1 || value > 50_000 {
            println!("value must be in [1,50000], given: {}", value);
            process::exit(1)
        }

        slice.push(value);
        if slice.len() == slice.capacity() {
            return (slice[0], slice[1]);
        }
    }

    println!("must be provided 2 values");
    process::exit(1)
}

fn read_segments(values: &mut Vec<Value>, number: usize) {
    let mut input = String::new();

    for _ in 0..number {
        io::stdin()
            .read_line(&mut input)
            .expect("can't read number");

        let mut slice: Vec<isize> = Vec::with_capacity(2);

        for val in input.trim().split_whitespace() {
            let value: isize = val.parse().unwrap();
            if value < - 100_000_000 || value > 100_000_000 {
                println!("value must be in [-10^8,10^8], given: {}", value);
                process::exit(1)
            }

            slice.push(value);

            if slice.len() == slice.capacity() {
                if slice[0] > slice[1] {
                    println!("start segment must be not greater then end, given [{},{}]", slice[0], slice[1]);
                    process::exit(1)
                }

                values.push(Value::new_edge(slice[0], true));
                values.push(Value::new_edge(slice[1], false));
                break;
            }
        }

        input = String::new();
    }
}

fn read_points(values: &mut Vec<Value>, number: usize) {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("can't read number");

    let mut i: usize = 0;
    for val in input.trim().split_whitespace() {
        let value: isize = val.parse().unwrap();
        if value < - 100_000_000 || value > 100_000_000 {
            println!("value must be in [-10^8,10^8], given: {}", value);
            process::exit(1)
        }

        values.push(Value::new_point(value, i));
        i += 1;
        if i == number {
            break;
        }
    }
}

fn run_lottery(items: &mut Vec<Value>) -> Vec<Point> {
    items.sort_by(|a, b| a.value().cmp(&b.value()));

    let mut opened: isize = 0;
    let mut points: Vec<Point> = Vec::new();

    for item in items.iter_mut() {
        match item {
            Value::Edge(e) => {
                opened += if e.is_start() { 1 } else { -1 };
            }
            Value::Point(p) => {
                points.push(Point::new(p.value, p.order, opened));
            }
        }
    }

    points
}

#[derive(Debug)]
enum Value {
    Point(Point),
    Edge(Edge),
}

impl Value {
    fn value(&self) -> isize {
        match self {
            Value::Edge(e) => e.value(),
            Value::Point(p) => p.value(),
        }
    }

    fn new_edge(value: isize, is_start: bool) -> Value { Value::Edge(Edge::new(value, is_start)) }
    fn new_point(value: isize, order: usize) -> Value { Value::Point(Point::new(value, order, 0)) }
}

#[derive(Debug)]
struct Point {
    value: isize,
    order: usize,
    segments: isize,
}

impl Point {
    fn new(value: isize, order: usize, segments: isize) -> Self { Point { value, order, segments } }
    fn order(&self) -> usize { self.order }
    fn value(&self) -> isize { self.value }
    fn segments(&self) -> isize { self.segments }
}

#[derive(Debug)]
struct Edge {
    value: isize,
    is_start: bool,
}

impl Edge {
    fn new(value: isize, is_start: bool) -> Self { Edge { value, is_start } }
    fn value(&self) -> isize { self.value }
    fn is_start(&self) -> bool { self.is_start }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_lottery() {
        let tests: Vec<(Vec<Value>, Vec<isize>)> = vec![
            (vec![
                Value::new_edge(0, true), Value::new_edge(5, false),
                Value::new_edge(7, true), Value::new_edge(10, false),
                Value::new_point(1, 0), Value::new_point(6, 1), Value::new_point(11, 2),
            ], vec![1, 0, 0]),
            (vec![
                Value::new_edge(-10, true), Value::new_edge(10, false),
                Value::new_point(-100, 0), Value::new_point(100, 1), Value::new_point(0, 2),
            ], vec![0, 0, 1]),
            (vec![
                Value::new_edge(0, true), Value::new_edge(5, false),
                Value::new_edge(-3, true), Value::new_edge(2, false),
                Value::new_edge(7, true), Value::new_edge(10, false),
                Value::new_point(1, 0), Value::new_point(6, 1)
            ], vec![2, 0]),
        ];

        for mut test in tests.into_iter() {
            let mut points = run_lottery(&mut test.0);

            points.sort_by(|a, b| a.order().cmp(&b.order()));

            let result = points.iter().map(|p| p.segments()).collect::<Vec<isize>>();

            assert_eq!(test.1, result);
        }
    }
}
