use std::io;

fn main() {
    let array = read_array();
    let elements = read_array();
    let mut result = String::new();

    for element in elements {
        if result.len() > 0 {
            result.push_str(&" ".to_string());
        }

        result.push_str(&binary_search(&array, element).to_string());
    }

    println!("{}", result);
}

fn binary_search(array: &Vec<usize>, element: usize) -> isize {
    let mut left: usize = 0;
    let mut right: usize = array.len();

    if right == 0 {
        return -1 as isize;
    }

    right = right - 1;

    while left < right {
        let mid = (left + right) >> 1;

        if array[mid] == element {
            return mid as isize;
        }

        if array[mid] > element {
            right = mid;
        } else {
            left = mid + 1;
        }
    }

    if array[left] == element {
        return left as isize;
    }

    return -1 as isize;
}

fn read_array() -> Vec<usize> {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("can't read numbers");

    let mut array: Vec<usize> = Vec::new();
    let mut i = 0;
    let mut size: usize = 0;

    for val in input.trim().split_whitespace() {
        i +=1;

        if i == 1 {
            size = val.parse().unwrap();
            continue;
        }

        if i > size + 1 {
            break;
        }

        array.push(val.parse().unwrap());
    }

    array
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_binary_search() {
        let tests: Vec<(Vec<usize>, usize, isize)> = vec![
            (vec![1, 5, 8, 12, 13], 8, 2),
            (vec![1, 5, 8, 12, 13], 1, 0),
            (vec![1, 5, 8, 12, 13], 23, -1),
            (vec![1, 5, 8, 12, 13], 11, -1),
            (vec![1, 5, 8, 12, 13], 12, 3),
        ];

        for test in tests.into_iter() {
            assert_eq!(test.2, binary_search(&test.0, test.1))
        }
    }
}