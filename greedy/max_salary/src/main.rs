use std::io;
use std::process;
use std::cmp::Ordering;

fn main() {
    let n = read_number();
    let nums = read_input(n);
    println!("{}", max_salary(nums));
}

fn max_salary(mut nums: Vec<usize>) -> String {
    nums.sort_by(|a, b| greater_or_equal(*b, *a));

    let mut result = String::new();
    for num in nums.iter() {
        result.push_str(&num.to_string());
    }

    result
}

fn greater_or_equal(mut a: usize, mut b: usize) -> Ordering {
    let len_a = len_usize(a);
    let len_b = len_usize(b);

    if len_a > len_b {
        for _ in len_b..len_a {
            b = b * 10 + 9;
        }
    }
    if len_a < len_b {
        for _ in len_a..len_b {
            a = a * 10 + 9;
        }
    }

    if a > b {
        return Ordering::Greater;
    }
    if a < b {
        return Ordering::Less;
    }
    return Ordering::Equal;
}

fn len_usize(n: usize) -> usize {
    let mut num = n;
    let mut len: usize = 1;
    loop {
        num = num / 10;
        if num == 0 {
            return len;
        }

        len += 1;
    }
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

fn read_input(n: usize) -> Vec<usize> {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("can't read number");

    let mut result: Vec<usize> = Vec::with_capacity(n);

    for val in input.trim().split_whitespace() {
        let value: usize = val.parse().unwrap();
        if value > 1_000 || value < 1 {
            println!("value must be in [0,10^9], given: {}", value);
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
    fn test_refills() {
        let tests: Vec<(Vec<usize>, String)> = vec![
            (vec![21, 2], String::from("221")),
            (vec![9, 4, 6, 1, 9], String::from("99641")),
            (vec![5, 523, 45], String::from("552345")),
        ];

        for test in tests.into_iter() {
            assert_eq!(test.1, max_salary(test.0))
        }
    }
}