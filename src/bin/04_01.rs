fn count_word<R: std::io::BufRead>(reader: R) -> i32 {
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();
    let target = "XMAS".as_bytes();
    let directions = [
        (0, 1),  // Right
        (1, 0),  // Down
        (0, usize::MAX), // Left (wrapping_add for decrement)
        (usize::MAX, 0), // Up (wrapping_add for decrement)
        (1, 1),  // Down-right
        (1, usize::MAX), // Down-left
        (usize::MAX, 1), // Up-right
        (usize::MAX, usize::MAX), // Up-left
    ];

    let mut count = 0;

    for row in 0..lines.len() {
        for col in 0..lines[0].len() {
            for &(row_dir, col_dir) in &directions {
                if matches_word(&lines, row, col, target, row_dir, col_dir) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn matches_word(
    lines: &[String],
    mut row: usize,
    mut col: usize,
    word: &[u8],
    row_dir: usize,
    col_dir: usize,
) -> bool {
    for &b in word {
        if let Some(line) = lines.get(row) {
            if let Some(&cell) = line.as_bytes().get(col) {
                if cell == b {
                    row = row.wrapping_add(row_dir);
                    col = col.wrapping_add(col_dir);
                    continue;
                }
            }
        }
        return false;
    }
    true
}

fn main() {
    let stdin = std::io::stdin();
    println!("{}", count_word(stdin.lock()));
}
