use regex::Regex;

fn count_sum<R: std::io::BufRead>(reader: R) -> i32 {
    let mut input = String::new();
    let mut buf_reader = reader;
    buf_reader.read_to_string(&mut input).unwrap();

    let regex =Regex::new("mul\\((\\d+),(\\d+)\\)|don't\\(\\)|do\\(\\)")
        .unwrap();

    regex
        .captures_iter(&input)
        .map(|caps| {
            let x = caps[1].parse::<i32>().unwrap();
            let y = caps[2].parse::<i32>().unwrap();
            x * y
        })
        .sum()
}

fn main() {
    println!("{}", count_sum(std::io::stdin().lock()));
}
