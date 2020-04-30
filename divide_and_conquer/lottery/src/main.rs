fn main() {
    println!("Hello, world!");
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
