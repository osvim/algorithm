use std::io;

fn main() {
    let first = read_line();
    let second = read_line();

    println!("{}", distance(&first, &second));
}

fn read_line() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("can't read string");

    input
}

fn distance(first: &str, second: &str) -> usize {
    let fc: Vec<char> = first.chars().collect();
    let sc: Vec<char> = second.chars().collect();

    let matrix = create_matrix(&fc, &sc);

    matrix[fc.len()][sc.len()]
}

// first chars - rows
// second chars - columns
fn create_matrix(first: &Vec<char>, second: &Vec<char>) -> Vec<Vec<usize>> {
    let mut matrix: Vec<Vec<usize>> = vec![
        vec![0; second.len() + 1];
        first.len() + 1
    ];

    for i in 0..first.len() + 1 {
        for j in 0..second.len() + 1 {
            if i == 0 {
                matrix[i][j] = j;
                continue;
            }
            if j == 0 {
                matrix[i][j] = i;
                continue;
            }

            let insertion = matrix[i][j - 1] + 1;
            let deletion = matrix[i - 1][j] + 1;
            let char_match = matrix[i - 1][j - 1];

            if first[i - 1] == second[j - 1] {
                matrix[i][j] = insertion.min(deletion).min(char_match);
            } else {
                let mismatch = char_match + 1;

                matrix[i][j] = insertion.min(deletion).min(mismatch);
            }
        }
    }

    matrix
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_distance() {
        let tests: Vec<(&str, &str, usize)> = vec![
            ("ab", "ab", 0),
            ("editing", "distance", 5),
            ("short", "ports", 3),
        ];

        for test in tests.into_iter() {
            assert_eq!(test.2, distance(test.0, test.1));
        }
    }
}