use std::io;
use std::process;

fn main() {
    let tup: (usize, usize) = read_input();
    println!("{}", fibonacci_modulo(tup.0, tup.1));
}

fn fibonacci_modulo(n: usize, m: usize) -> usize {
    let period = pisano_period(m);

    period[n % period.len()]
}

fn pisano_period(m: usize) -> Vec<usize> {
    let mut mods: Vec<usize> = Vec::new();
    mods.push(0);
    mods.push(1);

    let mut i = 2;
    loop {
        mods.push((mods[i - 1] + mods[i - 2]) % m);
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

        if numbers.len() == 0 && (number > 100_000_000_000_000 || number < 1) {
            println!("number must be in [1,10^14], given: {}", number);
            process::exit(1)
        }

        if numbers.len() == 1 && (number > 1000 || number < 2) {
            println!("mod must be in [2,1000], given: {}", number);
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
    fn test_pisano_period() {
        let tests: Vec<(usize, Vec<usize>)> = vec![
            (2, vec![0, 1, 1]),
            (3, vec![0, 1, 1, 2, 0, 2, 2, 1])
        ];

        for test in tests.into_iter() {
            assert_eq!(test.1, pisano_period(test.0))
        }
    }

    #[test]
    fn test_fibonacci_modulo() {
        let tests: Vec<(usize, usize, usize)> = vec![
            (239, 1000, 161),
            (2015, 3, 1),
            (2816213588, 239, 151),
        ];

        for test in tests.into_iter() {
            assert_eq!(test.2, fibonacci_modulo(test.0, test.1))
        }
    }
}
