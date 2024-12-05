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

            let is_valid = |seq: &[i32]| {
                let differences: Vec<i32> = seq
                    .windows(2)
                    .map(|pair| pair[1] - pair[0])
                    .collect();

                let is_increasing = differences.iter().all(|&diff| diff > 0);
                let is_decreasing = differences.iter().all(|&diff| diff < 0);
                let valid_differences = differences.iter().all(|&diff| diff.abs() >= 1 && diff.abs() <= 3);

                (is_increasing || is_decreasing) && valid_differences
            };

            if is_valid(&sequence) {
                return true;
            }

            (0..sequence.len()).any(|removed| {
                let mut levels = sequence.clone().into_iter().enumerate()
                    .filter_map(|(i, level)| if removed == i { None } else { Some(level) })
                    .collect::<Vec<_>>()
                    .into_iter();

                let start = levels.next().unwrap();
                let mut prev = levels.next().unwrap();
                if !(prev - start).abs() >= 1 && (prev - start).abs() <= 3 || prev == start {
                    return false;
                }

                let increasing = prev - start > 0;
                levels.all(|next| {
                    if !if increasing { 1..=3 } else { -3..=-1 }.contains(&(next - prev)) {
                        return false;
                    }
                    prev = next;
                    true
                })
            })
        })
        .count()
}

fn main() {
    let stdin = std::io::stdin();
    println!("{}", count_valid_lines(stdin.lock()));
}
