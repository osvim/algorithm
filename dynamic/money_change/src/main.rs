use std::io;
use std::process;

fn main() {
    let money = read_input();
    println!("{}", change_money(money));
}

static COINS: &'static [usize] = &[1, 3, 4];

fn change_money(money: usize) -> usize {
    let mut changes: Vec<usize> = vec![std::usize::MAX; money + 1];
    changes[0] = 0;

    for m in 1..money + 1 {
        for coin in COINS.iter() {
            if m >= *coin {
                let c = changes[m - *coin] + 1;
                if c < changes[m] {
                    changes[m] = c;
                }
            }
        }
    }

    return changes[money];
}

fn read_input() -> usize {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("can't read number");

    let money: usize = input.trim().parse().unwrap();
    if money > 1000 || money < 1 {
        println!("money value must be in [1,10^3], given: {}", money);
        process::exit(1)
    }

    money
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_change_money() {
        let tests = vec![(2, 2), (0, 0), (34, 9)];

        for test in tests.into_iter() {
            assert_eq!(test.1, change_money(test.0));
        }
    }
}