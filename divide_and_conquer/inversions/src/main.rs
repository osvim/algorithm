use std::io;
use std::process;

fn main() {
    let mut vec = read_slice(read_size());
    let ins = inversions(vec.as_mut_slice());
    println!("{}", ins)
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

fn read_slice(size: usize) -> Vec<usize> {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("can't read number");

    let mut slice: Vec<usize> = Vec::with_capacity(size);

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

// merge sort with inversions counter, returns number of inversions
fn inversions(slice: &mut[usize]) -> usize {
    let length = slice.len();
    if length < 2 {
        return 0;
    }

    let half = slice.len() >> 1;
    let (first_half, second_half) = slice.split_at_mut(half);
    let mut ins: usize = inversions(first_half) + inversions(second_half);
    let mut sorted = Vec::with_capacity(length);
    let mut first: usize = 0;
    let mut second: usize = half;

    // println!("=============================================================");
    // println!("first half = {:?} second half = {:?} inversions = {}", first_half, second_half, ins);

    while first < half && second < slice.len() {
        // println!("=>  compare {} to {}:", slice[first], slice[second]);
        if slice[first] > slice[second] {
            sorted.push(slice[second]);
            second += 1;
            ins += half - first;
            // println!("=>  less --> ins {}", half - first);
        } else {
            sorted.push(slice[first]);
            first += 1;
            // println!("=>  not less");
        }
        // println!("=>  done");
    }

    for i in first..half {
        sorted.push(slice[i]);
    }

    for i in second..slice.len() {
        sorted.push(slice[i]);
    }

    // println!("inversions({:?}) = {} --> {:?}", slice, ins, sorted);
    // println!("=============================================================");

    slice.copy_from_slice(sorted.as_slice());

    return ins;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inversions() {
        let tests: Vec<(Vec<usize>, usize)> = vec![
            (vec![1, 2, 3, 4, 5], 0),
            (vec![2, 3, 9, 2, 9], 2),
            (vec![5, 4, 3, 2, 1], 10),
            (vec![2, 2, 1, 2, 2], 2),
        ];

        for mut test in tests.into_iter() {
            assert_eq!(test.1, inversions(test.0.as_mut_slice()));
        }
    }
}
