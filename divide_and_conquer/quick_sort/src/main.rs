extern crate rand;

use std::io;
use std::process;
use rand::Rng;
use std::cmp::Ordering;

fn main() {
    let mut vec = read_slice(read_size());
    quick_sort(vec.as_mut_slice());

    let mut result = String::new();
    for e in vec.iter() {
        if result.len() > 0 {
            result.push_str(" ");
        }

        result.push_str(&e.to_string());
    }

    println!("{}", result);
}

fn read_size() -> usize {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("can't read number");

    let size: usize = input.trim().parse().unwrap();
    if size > 100_000 || size < 1 {
        println!("size must be in [1,10^5], given: {}", size);
        process::exit(1)
    }

    size
}

fn read_slice(size: usize) -> Vec<u64> {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("can't read number");

    let mut slice: Vec<u64> = Vec::with_capacity(size);

    for val in input.trim().split_whitespace() {
        let value = val.parse().unwrap();
        if value < 1 || value > 1_000_000_000 {
            println!("value must be in [1,10^9], given: {}", value);
            process::exit(1)
        }

        slice.push(value);
        if slice.len() == slice.capacity() {
            break;
        }
    }

    slice
}

fn quick_sort<T>(slice: &mut [T]) where T: Ord {
    let length: usize = slice.len();
    if length < 2 {
        return;
    }

    let mut rng = rand::thread_rng();
    slice.swap(0, rng.gen_range(0, length));

    let mut eq: usize = 0;
    let mut pointer: usize = 0;

    for i in 1..length {
        match &slice[i].cmp(&slice[0]) {
            Ordering::Greater => (),
            Ordering::Equal => {
                pointer += 1;
                slice.swap(pointer, i);
                eq += 1;
            }
            Ordering::Less => {
                pointer += 1;
                slice.swap(pointer, i);

                if eq > 0 {
                    slice.swap(pointer - eq, pointer)
                }
            }
        }
    }

    slice.swap(pointer - eq, 0);

    let (less, equal_or_greater) = slice.split_at_mut(pointer - eq);
    let (_, greater) = equal_or_greater.split_at_mut(eq);

    quick_sort(less);
    quick_sort(greater);
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;

    #[test]
    fn test_quick_sort() {
        let mut slice: Vec<u64> = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..100_000{
            slice.push(rng.gen_range(0, u64::MAX))
        }

        quick_sort(slice.as_mut_slice());

        for i in 1..100_000 {
            assert!(slice[i] >= slice[i - 1]);
        }
    }
}