use std::io;
use std::process;
use std::fmt;
use std::slice::Iter;

fn main() {
    let (operations, sequence) = calculate(read_input());
    println!("{}", operations);
    println!("{}", sequence);
}

fn read_input() -> usize {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("can't read number");

    let number: usize = input.trim().parse().unwrap();
    if number > 1_000_000 || number < 1 {
        println!("number value must be in [1,10^6], given: {}", number);
        process::exit(1)
    }

    number
}

enum Operation {
    INCREMENT,
    X2,
    X3,
}

impl Operation {
    fn previous(&self, n: usize) -> (usize, bool) {
        match *self {
            Operation::INCREMENT => if n > 0 { (n - 1, true) } else { (0, false) },
            Operation::X2 => if n % 2 == 0 { (n / 2, true) } else { (0, false) },
            Operation::X3 => if n % 3 == 0 { (n / 3, true) } else { (0, false) },
        }
    }

    pub fn iter() -> Iter<'static, Operation> {
        static OPERATIONS: [Operation; 3] = [Operation::INCREMENT, Operation::X2, Operation::X3];
        OPERATIONS.iter()
    }
}

impl fmt::Display for Operation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Operation::INCREMENT => write!(f, "+1"),
            Operation::X2 => write!(f, "*2"),
            Operation::X3 => write!(f, "*3"),
        }
    }
}

fn calculate(number: usize) -> (usize, String) {
    let mut operations: Vec<usize> = vec![std::usize::MAX; number + 1];
    let mut rows: Vec<String> = vec![String::new(); number + 1];
    operations[0] = 0;

    for n in 1..number + 1 {
        for operation in Operation::iter() {
            let (previous, supports) = operation.previous(n);
            if supports {
                let o = operations[previous] + 1;
                if o < operations[n] {
                    operations[n] = if n == 1 { 0 } else { o };
                    rows[n] = rows[previous].clone();
                    if rows[n].len() > 0 {
                        rows[n].push(' ');
                    }
                    rows[n].push_str(&n.to_string());
                }
            }
        }
    }

    return (operations[number], rows[number].clone());
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_calculate() {
        let tests: Vec<(usize, usize, String)> = vec![
            (1, 0, String::from("1")),
            (5, 3, String::from("1 3 4 5")),
            (8, 3, String::from("1 3 4 8")),
            (16, 4, String::from("1 3 4 8 16")),
            (1024, 10, String::from("1 3 9 27 28 84 85 255 256 512 1024")),
            (8019, 10, String::from("1 3 9 10 11 33 99 297 891 2673 8019")),
            (96234, 14, String::from("1 3 9 10 11 33 99 297 891 2673 8019 16038 16039 48117 96234"))
        ];

        for test in tests.into_iter() {
            let res: (usize, String) = calculate(test.0);
            assert_eq!(test.2, res.1);
            assert_eq!(test.1, res.0);
        }
    }
}