use std::io;
use std::process;

fn main() {
    let tup: (usize, usize) = read_input();
    println!("{:?}", lcm(tup.0, tup.1));
}
fn lcm(first: usize, second: usize) -> usize {
    return first * second / gcd(first, second);
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn read_input() -> (usize, usize) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("can't read numbers");

    let mut numbers: Vec<usize> = Vec::with_capacity(2);

    for val in input.trim().split_whitespace() {
        let number = val.parse().unwrap();
        if number > 10_000_000 {
            println!("number must be in [0,10^7], given: {}", number);
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
    fn test_fibonacci() {
        let tests: Vec<(usize, usize, usize)> = vec![
            (6,8,24),
            (3, 5, 15),
            (761457, 614573, 467970912861)
        ];

        for test in tests.into_iter() {
            assert_eq!(test.2, lcm(test.0, test.1))
        }
    }
}