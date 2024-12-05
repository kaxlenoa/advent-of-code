use regex::Regex;

fn count_sum(mut reader: impl std::io::BufRead) -> i32 {
    let mut string = String::new();
    reader.read_to_string(&mut string).unwrap();
    let mut dont = false;
    Regex::new(r"mul\((\d+),(\d+)\)|don't\(\)|do\(\)")
        .unwrap()
        .captures_iter(&string)
        .map(|capture| match capture.get(0).unwrap().as_str() {
            "do()" => {
                dont = false;
                0
            }
            "don't()" => {
                dont = true;
                0
            }
            _ => {
                if !dont {
                    let x = capture.get(1).unwrap().as_str().parse::<i32>().unwrap();
                    let y = capture.get(2).unwrap().as_str().parse::<i32>().unwrap();
                    x * y
                } else {
                    0
                }
            }
        })
        .sum()
}

fn main() {
    println!("{}", count_sum(std::io::stdin().lock()));
}