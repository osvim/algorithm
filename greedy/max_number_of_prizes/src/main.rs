use std::io;
use std::process;

fn main() {
    let prizes = max_number_of_prizes(read_input());
    println!("{}", prizes.len());
    let mut result = String::new();
    for (i,point) in prizes.iter().enumerate() {
        result.push_str(&point.to_string());
        if i < prizes.len() {
            result.push_str(&String::from(" "));
        }
    }
    println!("{}", result);
}

fn max_number_of_prizes(mut n: usize) -> Vec<usize> {
    if n < 2 {
        return vec![n];
    }

    let mut prev: usize = 0;
    let mut prizes: Vec<usize> = Vec::new();

    loop {
        if prev < ((n - 1) >> 1) {
            prev += 1;
            prizes.push(prev);
            n -= prev;
        } else {
            prizes.push(n);
            return prizes;
        }
    }
}

fn read_input() -> usize {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("can't read number");

    let n: usize = input.trim().parse().unwrap();
    if n > 1_000_000_000 || n < 1 {
        println!("number must be in [1,10^9], given: {}", n);
        process::exit(1)
    }

    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_revenue() {
        let tests: Vec<(usize, Vec<usize>)> = vec![
            (6, vec![1, 2, 3]),
            (8, vec![1, 2, 5]),
            (2, vec![2]),
            (45, vec![1, 2, 3, 4, 5, 6, 7, 8, 9]),
        ];

        for test in tests.into_iter() {
            assert_eq!(test.1, max_number_of_prizes(test.0))
        }
    }
}