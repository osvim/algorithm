use std::io;
use std::process;

fn main() {
    let digit = fibonacci_last_digit(read_input());
    println!("{}", digit);
}

fn fibonacci_last_digit(n: usize) -> usize {
    if n < 2 {
        return n;
    }

    let mut numbers: Vec<usize> = vec![0; n + 1];
    numbers[1] = 1;
    for i in 2..n + 1 {
        numbers[i] = (numbers[i - 1] + numbers[i - 2]) % 10
    }

    numbers[n]
}

fn read_input() -> usize {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("can't read number");

    let number: usize = input.trim().parse().unwrap();

    if number > 10_000_000 {
        println!("number must be in [0,10^7]");
        process::exit(1);
    }

    number
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_fibonacci() {
        let tests: Vec<(usize, usize)> = vec![(0, 0), (1, 1), (10, 5), (45, 0), (331, 9)];

        for test in tests.into_iter() {
            assert_eq!(test.1, fibonacci_last_digit(test.0))
        }
    }
}