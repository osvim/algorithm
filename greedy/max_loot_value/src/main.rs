use std::io;
use std::process;
use std::cmp::min;

fn main() {
    let mut knapsack = read_input();
    println!("{:.4}", knapsack.max_value());
}

struct Item {
    value: usize,
    weight: usize,
}

impl Item {
    fn fraction_value(&self) -> f64 {
        self.value as f64 / self.weight as f64
    }

    fn new(value: usize, weight: usize) -> Item {
        Item { value, weight }
    }
}

struct Knapsack {
    capacity: usize,
    loot: Vec<Item>,
}

impl Knapsack {
    fn max_value(&mut self) -> f64 {
        self.loot.sort_by(|a, b| b.fraction_value().partial_cmp(&a.fraction_value()).unwrap());

        let mut value: f64 = 0.0;
        let mut capacity: usize = self.capacity;

        for item in self.loot.iter() {
            let cap = min(capacity, item.weight);
            capacity -= cap;

            value += cap as f64 * item.fraction_value();

            if capacity == 0 {
                break;
            }
        }

        value
    }
}

fn read_input() -> Knapsack {
    let mut knapsack = read_knapsack();
    for _ in 0..knapsack.loot.capacity() {
        knapsack.loot.push(read_item())
    }

    knapsack
}

fn read_item() -> Item {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("can't read numbers");

    let mut numbers: Vec<usize> = Vec::with_capacity(2);

    for val in input.trim().split_whitespace() {
        let number = val.parse().unwrap();

        if numbers.len() == 0 && number > 2_000_000 {
            println!("value of item must be in [0,2000000], given: {}", number);
            process::exit(1)
        }

        if numbers.len() == 1 && number > 2_000_000 {
            println!("weight of item must be in [0,2000000], given: {}", number);
            process::exit(1)
        }

        numbers.push(number);
        if numbers.len() == numbers.capacity() {
            break;
        }
    }

    if numbers.len() < numbers.capacity() {
        println!("number of items and capacity of knapsack must be provided");
        process::exit(1)
    }

    Item::new(numbers[0], numbers[1])
}

fn read_knapsack() -> Knapsack {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("can't read numbers");

    let mut numbers: Vec<usize> = Vec::with_capacity(2);

    for val in input.trim().split_whitespace() {
        let number = val.parse().unwrap();

        if numbers.len() == 0 && (number > 1_000 || number < 1) {
            println!("number of items must be in [1,1000], given: {}", number);
            process::exit(1)
        }

        if numbers.len() == 1 && number > 2_000_000 {
            println!("capacity of knapsack must be in [0,2000000], given: {}", number);
            process::exit(1)
        }

        numbers.push(number);
        if numbers.len() == numbers.capacity() {
            break;
        }
    }

    if numbers.len() < numbers.capacity() {
        println!("number of items and capacity of knapsack must be provided");
        process::exit(1)
    }

    Knapsack {
        capacity: numbers[1],
        loot: Vec::with_capacity(numbers[0]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_item_fraction_value() {
        let tests: Vec<(Item, f64)> = vec![
            (Item { value: 50, weight: 10 }, 5.0),
            (Item { value: 10, weight: 4 }, 2.5),
        ];

        for test in tests.into_iter() {
            assert_eq!(test.1, test.0.fraction_value())
        }
    }

    #[test]
    fn test_knapsack_max_value() {
        let tests: Vec<(Knapsack, f64)> = vec![
            (Knapsack { capacity: 10, loot: vec![Item { value: 500, weight: 30 }] }, 500.0 / 3.0),
            (Knapsack { capacity: 50, loot: vec![Item { value: 60, weight: 20 }, Item { value: 100, weight: 50 }, Item { value: 120, weight: 30 }] }, 180.0),
        ];

        for mut test in tests.into_iter() {
            assert_eq!(format!("{:.4}", test.1),format!("{:.4}", test.0.max_value()))
        }
    }
}