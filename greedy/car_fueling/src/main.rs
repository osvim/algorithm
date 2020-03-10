use std::io;
use std::process;

fn main() {
    let mut problem = read_input();
    println!("{}", problem.refills());
}

struct Problem {
    distance: usize,
    capacity: usize,
    stops: Vec<usize>,
}

impl Problem {
    fn refills(&mut self) -> isize {
        self.stops.push(self.distance);
        self.stops.sort_by(|a, b| a.partial_cmp(&b).unwrap());

        let mut refills: isize = 0;
        let mut pos: usize = 0;
        let mut last: usize = pos;

        for stop in self.stops.iter() {
            if *stop <= self.capacity + last {
                pos = *stop;
                continue;
            }

            if last < pos {
                last = pos;
                refills += 1;
                continue;
            }
            return -1;
        }

        refills
    }
}

fn read_input() -> Problem {
    let distance = read_distance();
    let capacity = read_capacity();
    let amount = read_stops_amount();
    let stops = read_stops(distance, amount);

    Problem {
        distance: distance,
        capacity: capacity,
        stops: stops,
    }
}

fn read_distance() -> usize {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("can't read number");

    let distance: usize = input.trim().parse().unwrap();
    if distance > 100_000 || distance < 1 {
        println!("distance must be in [1,10^5], given: {}", distance);
        process::exit(1)
    }

    distance
}

fn read_capacity() -> usize {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("can't read number");

    let capacity: usize = input.trim().parse().unwrap();
    if capacity > 400 || capacity < 1 {
        println!("tank volume must be in [1,400], given: {}", capacity);
        process::exit(1)
    }

    capacity
}

fn read_stops_amount() -> usize {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("can't read number");

    let amount: usize = input.trim().parse().unwrap();
    if amount > 300 || amount < 1 {
        println!("number of stops must be in [1,300], given: {}", amount);
        process::exit(1)
    }

    amount
}

fn read_stops(distance: usize, amount: usize) -> Vec<usize> {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("can't read number");

    let mut stops: Vec<usize> = Vec::new();
    let mut last: usize = 0;
    let mut i = amount;

    for val in input.trim().split_whitespace() {
        if i == 0 {
            break;
        }

        let stop = val.parse().unwrap();
        if stop > distance || stop < 1 {
            println!("stop must be less than distance, given: {}", stop);
            process::exit(1)
        }

        if last > 0 && stop <= last {
            println!("stop[i] must be less then stop[i+1]");
            process::exit(1)
        }

        stops.push(stop);
        i -= 1;
        last = stop;
    }

    stops
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_refills() {
        let tests: Vec<(Problem, isize)> = vec![
            (Problem { distance: 900, capacity: 400, stops: vec![200, 375, 550, 750] }, 2),
            (Problem { distance: 10, capacity: 3, stops: vec![1, 2, 5, 9] }, -1),
            (Problem { distance: 200, capacity: 250, stops: vec![100, 150] }, 0),
        ];

        for mut test in tests.into_iter() {
            assert_eq!(test.1, test.0.refills())
        }
    }
}