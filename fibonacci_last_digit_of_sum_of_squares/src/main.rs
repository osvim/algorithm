use std::io;
use std::process;

fn main() {
    println!("{}", fibonacci_last_digit_of_sum_of_squares(read_input()));
}

// f(0)^2 + f(1)^2 + ... + f(n)^2 = f(0)^2 + f(n)*f(n + 1) = f(n)*f(n + 1)
fn fibonacci_last_digit_of_sum_of_squares(n: usize) -> usize {
    let period = pisano_period();

    period[n % period.len()] * period[(n + 1) % period.len()] % 10
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

fn read_input() -> usize {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("can't read number");

    let number: usize = input.trim().parse().unwrap();

    if number > 100_000_000_000_000 {
        println!("number must be in [0,10^14]");
        process::exit(1);
    }

    number
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fibonacci_last_digit_of_interval() {
        let tests: Vec<(usize, usize)> = vec![(7, 3), (73, 1), (1234567890, 0)];

        for test in tests.into_iter() {
            assert_eq!(test.1, fibonacci_last_digit_of_sum_of_squares(test.0))
        }
    }
}
