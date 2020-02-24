use std::io;

fn main() {
    let max_product = max_pairwise_product(read_input());
    println!("{}", max_product)
}

fn max_pairwise_product(ints: Vec<i64>) -> i64 {
    let mut first: i64 = 0;
    let mut second: i64 = 0;

    for val in ints.iter() {
        if *val > first {
            second = first;
            first = *val;
            continue;
        }

        if *val > second {
            second = *val;
        }
    }

    first * second
}

fn read_input() -> Vec<i64> {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("can't read size");

    let mut size: usize = input.trim().parse().unwrap();
    let mut result: Vec<i64> = Vec::with_capacity(size);

    input = "".to_string();
    io::stdin().read_line(&mut input).expect("can't read vector");

    for val in input.trim().split_whitespace() {
        if size == 0 {
            break;
        }

        result.push(val.parse().unwrap());
        size -= 1;
    }

    result
}

#[cfg(test)]
mod tests {
    extern crate rand;

    use super::*;
    use rand::Rng;
    use std::time::{Duration, SystemTime};

    #[test]
    fn test_max_pairwise_product() {
        let tests: Vec<(Vec<i64>, i64)> = vec![
            (vec![1, 2, 3], 6),
            (vec![10, 1, 10], 100),
            (vec![9, 0, 0, 0], 0),
            (vec![0, 0, 0, 0], 0),
        ];

        for test in tests.into_iter() {
            assert_eq!(test.1, max_pairwise_product(test.0))
        }
    }

    #[test]
    fn stress_max_pairwise_product() {
        let time = SystemTime::now();
        let one_sec = Duration::from_secs(1);

        let _ = max_pairwise_product(gen_long_vec());

        assert!(time.elapsed().unwrap() <= one_sec);
    }

    fn gen_long_vec() -> Vec<i64> {
        let cap = 100_000;
        let mut rng = rand::thread_rng();
        let mut ints = Vec::<i64>::with_capacity(cap);

        for _ in 0..cap {
            ints.push(rng.gen_range(1, 1_000_000));
        }

        ints
    }
}