fn count_valid_lines(input: impl std::io::BufRead) -> usize {
    input.lines()
        .filter_map(|line| line.ok())
        .filter(|line| {
            let sequence: Vec<i32> = line
                .split_whitespace()
                .filter_map(|num| num.parse::<i32>().ok())
                .collect();

            if sequence.len() < 2 {
                return false;
            }

            let differences: Vec<i32> = sequence
                .windows(2)
                .map(|pair| pair[1] - pair[0])
                .collect();

            let is_increasing = differences.iter().all(|&diff| diff > 0);
            let is_decreasing = differences.iter().all(|&diff| diff < 0);

            let valid_differences = differences.iter().all(|&diff| diff.abs() >= 1 && diff.abs() <= 3);

            (is_increasing || is_decreasing) && valid_differences
        })
        .count()
}

fn main() {
    let stdin = std::io::stdin();
    println!("{}", count_valid_lines(stdin.lock()));
}
