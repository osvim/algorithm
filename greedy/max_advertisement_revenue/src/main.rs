use std::io;
use std::process;

fn main() {
    let mut problem = read_input();
    println!("{}", problem.max_revenue());
}

struct Problem {
    profits: Vec<isize>,
    clicks: Vec<isize>,
}

impl Problem {
    fn max_revenue(&mut self) -> isize {
        self.profits.sort_by(|a, b| a.partial_cmp(&b).unwrap());
        self.clicks.sort_by(|a, b| a.partial_cmp(&b).unwrap());

        let mut revenue: isize = 0;

        let len_profits = self.profits.len();
        for (i, click) in self.clicks.iter().enumerate() {
            if i == len_profits {
                break;
            }

            revenue += click * self.profits[i];
        }

        revenue
    }
}

fn read_input() -> Problem {
    let n = read_number();
    Problem {
        clicks: read_vec(n),
        profits: read_vec(n),
    }
}

fn read_number() -> usize {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("can't read number");

    let n: usize = input.trim().parse().unwrap();
    if n > 1000 || n < 1 {
        println!("number must be in [1,10^3], given: {}", n);
        process::exit(1)
    }

    n
}

fn read_vec(n: usize) -> Vec<isize> {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("can't read number");

    let mut result: Vec<isize> = Vec::with_capacity(n);

    for val in input.trim().split_whitespace() {
        let value: isize = val.parse().unwrap();
        if value > 100_000 || value < -100_000 {
            println!("value must be in [-10^5,10^5], given: {}", value);
            process::exit(1)
        }

        result.push(value);
        if result.len() == result.capacity() {
            break
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_revenue() {
        let tests: Vec<(Problem, isize)> = vec![
            (Problem { clicks: vec![23], profits: vec![39] }, 897),
            (Problem { clicks: vec![1, 3, -5], profits: vec![-2, 4, 1] }, 23),
        ];

        for mut test in tests.into_iter() {
            assert_eq!(test.1, test.0.max_revenue())
        }
    }
}