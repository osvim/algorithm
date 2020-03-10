use std::io;
use std::process;

fn main() {
    let intersection_points = read_input().intersection_points();
    println!("{}", intersection_points.len());
    let mut result = String::new();
    for (i,point) in intersection_points.iter().enumerate() {
        result.push_str(&point.to_string());
        if i < intersection_points.len() {
            result.push_str(&String::from(" "));
        }
    }
    println!("{}", result);
}

struct Segment {
    start: usize,
    end: usize,
}

struct Problem {
    segments: Vec<Segment>,
}

impl Segment {
    fn new(a: usize, b: usize) -> Segment {
        Segment { start: a, end: b }
    }
}

impl Problem {
    fn intersection_points(&mut self) -> Vec<usize> {
        self.segments.sort_by(|a, b| a.start.partial_cmp(&b.start).unwrap());

        let mut intersections: Vec<usize> = Vec::new();
        let mut point: usize = 0;
        let mut set: bool = false;

        for segment in self.segments.iter() {
            if !set {
                point = segment.end;
                set = true;
                continue;
            }

            if segment.start > point {
                intersections.push(point);
                set = false;
            }
        }

        if set {
            intersections.push(point);
        }

        intersections
    }
}

fn read_input() -> Problem {
    let mut segments: Vec<Segment> = Vec::new();
    for _ in 0..read_number() {
        segments.push(read_segment());
    }

    Problem{segments}
}

fn read_number() -> usize {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("can't read number");

    let n: usize = input.trim().parse().unwrap();
    if n > 100 || n < 1 {
        println!("number must be in [1,100], given: {}", n);
        process::exit(1)
    }

    n
}

fn read_segment() -> Segment {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("can't read number");

    let mut result: Vec<usize> = Vec::with_capacity(2);

    for val in input.trim().split_whitespace() {
        let value: usize = val.parse().unwrap();
        if value > 1_000_000_000 {
            println!("value must be in [0,10^9], given: {}", value);
            process::exit(1)
        }

        result.push(value);
        if result.len() == result.capacity() {
            break
        }
    }

    Segment::new(result[0], result[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_refills() {
        let tests: Vec<(Problem, Vec<usize>)> = vec![
            (Problem { segments: vec![Segment::new(1, 3), Segment::new(3, 6), Segment::new(2, 5)] }, vec![3]),
            (Problem { segments: vec![Segment::new(4, 7), Segment::new(1, 3), Segment::new(2, 5), Segment::new(5, 6)] }, vec![3, 6]),
        ];

        for mut test in tests.into_iter() {
            assert_eq!(test.1, test.0.intersection_points())
        }
    }
}