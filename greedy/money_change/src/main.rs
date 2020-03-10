use std::io;
use std::process;

static DENOMINATIONS: [usize; 3] = [10, 5, 1];

fn main() {
    println!("{}", change_money(read_input()));
}

fn change_money(input: usize) -> usize {
    let mut coins: usize = 0;
    let mut rest = input;

    for coin in DENOMINATIONS.iter() {
        coins += rest / coin;
        rest = rest % coin;
        if rest == 0 {
            break
        }
    }

    coins
}

fn read_input() -> usize {
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("can't read number");

    let number: usize = input.trim().parse().unwrap();

    if number > 1000 {
        println!("number must be in [0,1000]");
        process::exit(1);
    }

    number
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_change_money() {
        let tests: Vec<(usize, usize)> = vec![(2,2),(28,6),(0,0),(100,10)];

        for test in tests.into_iter() {
            assert_eq!(test.1, change_money(test.0))
        }
    }
}