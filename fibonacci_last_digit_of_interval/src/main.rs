use std::io;
use std::process;

fn main() {
    let tup: (usize, usize) = read_input();
    println!("{}", fibonacci_last_digit_of_interval(tup.0, tup.1));
}

// f(0) + f(1) + ... + f(n) = f(n + 2) - 1
// f(m) + f(m + 1) + ... + f(n) = f(n + 2) - 1 - (f(m - 1 + 2) - 1) = f(n + 2) - f(m + 1)
// last digit of sum = f(n + 2) mod 10 - f(m + 1) mod 10
// if last digit < 0 , last digit += 10
fn fibonacci_last_digit_of_interval(n: usize, m: usize) -> usize {
    if n > m {
        return calc_fibonacci_last_digit_of_interval(n, m);
    }

    return calc_fibonacci_last_digit_of_interval(m, n);
}

fn calc_fibonacci_last_digit_of_interval(n: usize, m: usize) -> usize {
    let period = pisano_period();

    let f = period[(n + 2) % period.len()];
    let s = period[(m + 1) % period.len()];

    if f < s {
        return 10 - (s - f) % 10;
    }

    return f - s;
}

// period for modulo 10
fn pisano_period() -> Vec<usize> {
    let mut mods: Vec<usize> = Vec::new();
    mods.push(0);
    mods.push(1);

    let mut i = 2;
    loop {
        mods.push((mods[i - 1] + mods[i - 2]) % 10);
        if mods[i] == 1 && mods[i - 1] == 0 {
            break;
        }

        i += 1
    }

    (&mods[..i - 1]).to_vec()
}

fn read_input() -> (usize, usize) {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("can't read numbers");

    let mut numbers: Vec<usize> = Vec::with_capacity(2);

    for val in input.trim().split_whitespace() {
        let number = val.parse().unwrap();
        if number > 100_000_000_000_000 {
            println!("number must be in [0,10^14], given: {}", number);
            process::exit(1)
        }

        numbers.push(number);
        if numbers.len() == numbers.capacity() {
            break;
        }
    }

    if numbers.len() < numbers.capacity() {
        println!("2 numbers must be provided");
        process::exit(1)
    }

    (numbers[0], numbers[1])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_last_digit_of_interval() {
        let tests: Vec<(usize, usize, usize)> = vec![
            (3, 7, 1),
            (10, 10, 5),
            (10, 200, 2),
        ];

        for test in tests.into_iter() {
            assert_eq!(test.2, fibonacci_last_digit_of_interval(test.0, test.1))
        }
    }
}
