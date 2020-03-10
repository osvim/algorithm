use std::io;
use std::process;

fn main() {
    let intersection_points = read_input().intersection_points();
    println!("{}", intersection_points.len());
    let mut result = String::new();
    for (i, point) in intersection_points.iter().enumerate() {
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
            if segment.start > point && point > 0 {
                intersections.push(point);
                set = false;
            }

            if !set {
                point = segment.end;
                set = true;
                continue;
            }

            if segment.end < point {
                point = segment.end;
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

    Problem { segments }
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
            break;
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
            (Problem {
                segments: vec![
                    Segment::new(41, 42),
                    Segment::new(52, 52),
                    Segment::new(63, 63),
                    Segment::new(80, 82),
                    Segment::new(78, 79),
                    Segment::new(35, 35),
                    Segment::new(22, 23),
                    Segment::new(31, 32),
                    Segment::new(44, 45),
                    Segment::new(81, 82),
                    Segment::new(36, 38),
                    Segment::new(10, 12),
                    Segment::new(1, 1),
                    Segment::new(23, 23),
                    Segment::new(32, 33),
                    Segment::new(87, 88),
                    Segment::new(55, 56),
                    Segment::new(69, 71),
                    Segment::new(89, 91),
                    Segment::new(93, 93),
                    Segment::new(38, 40),
                    Segment::new(33, 34),
                    Segment::new(14, 16),
                    Segment::new(57, 59),
                    Segment::new(70, 72),
                    Segment::new(36, 36),
                    Segment::new(29, 29),
                    Segment::new(73, 74),
                    Segment::new(66, 68),
                    Segment::new(36, 38),
                    Segment::new(1, 3),
                    Segment::new(49, 50),
                    Segment::new(68, 70),
                    Segment::new(26, 28),
                    Segment::new(30, 30),
                    Segment::new(1, 2),
                    Segment::new(64, 65),
                    Segment::new(57, 58),
                    Segment::new(58, 58),
                    Segment::new(51, 53),
                    Segment::new(41, 41),
                    Segment::new(17, 18),
                    Segment::new(45, 46),
                    Segment::new(4, 4),
                    Segment::new(0, 1),
                    Segment::new(65, 67),
                    Segment::new(92, 93),
                    Segment::new(84, 85),
                    Segment::new(75, 77),
                    Segment::new(39, 41),
                    Segment::new(15, 15),
                    Segment::new(29, 31),
                    Segment::new(83, 84),
                    Segment::new(12, 14),
                    Segment::new(91, 93),
                    Segment::new(83, 84),
                    Segment::new(81, 81),
                    Segment::new(3, 4),
                    Segment::new(66, 67),
                    Segment::new(8, 8),
                    Segment::new(17, 19),
                    Segment::new(86, 87),
                    Segment::new(44, 44),
                    Segment::new(34, 34),
                    Segment::new(74, 74),
                    Segment::new(94, 95),
                    Segment::new(79, 81),
                    Segment::new(29, 29),
                    Segment::new(60, 61),
                    Segment::new(58, 59),
                    Segment::new(62, 62),
                    Segment::new(54, 56),
                    Segment::new(58, 58),
                    Segment::new(79, 79),
                    Segment::new(89, 91),
                    Segment::new(40, 42),
                    Segment::new(2, 4),
                    Segment::new(12, 14),
                    Segment::new(5, 5),
                    Segment::new(28, 28),
                    Segment::new(35, 36),
                    Segment::new(7, 8),
                    Segment::new(82, 84),
                    Segment::new(49, 51),
                    Segment::new(2, 4),
                    Segment::new(57, 59),
                    Segment::new(25, 27),
                    Segment::new(52, 53),
                    Segment::new(48, 49),
                    Segment::new(9, 9),
                    Segment::new(10, 10),
                    Segment::new(78, 78),
                    Segment::new(26, 26),
                    Segment::new(83, 84),
                    Segment::new(22, 24),
                    Segment::new(86, 87),
                    Segment::new(52, 54),
                    Segment::new(49, 51),
                    Segment::new(63, 64),
                    Segment::new(54, 54)
                ]
            },
             vec![1, 4, 5, 8, 9, 10, 14, 15, 18, 23, 26, 28, 29, 30, 32, 34, 35, 36, 40, 41, 44, 46, 49, 52, 54, 56, 58, 61, 62, 63, 65, 67, 70, 74, 77, 78, 79, 81, 84, 87, 91, 93, 95]
            ),
        ];
        for mut test in tests.into_iter() {
            assert_eq!(test.1, test.0.intersection_points())
        }
    }
}