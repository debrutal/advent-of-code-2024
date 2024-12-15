use lazy_static::lazy_static;
use regex::Regex; // 1.3.0
use std::env;

lazy_static! {
    static ref RE: Regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    static ref DONT: Regex = Regex::new(r"don't()").unwrap();
    static ref DO: Regex = Regex::new(r"do()").unwrap();
}

fn main() {
    println!("Current directory: {:?}", env::current_dir().unwrap());

    let input = read_input();
    println!("read {} lines", input.len());
    let mut sum = 0;
    for line in input {
        sum += parse(&line);
    }
    println!("Sum: {}", sum);
}

fn read_input() -> Vec<String> {
    let input = std::fs::read_to_string("../input/input.txt").expect("could not open file");
    input.lines().map(|s| s.to_string()).collect()
}
fn parse(line: &String) -> u32 {
    // Iterate over all matches
    let mut sum_of_line: u32 = 0;
    for cap in RE.captures_iter(line) {
        let first = &cap[1]; // The first number
        let second = &cap[2]; // The second number
        print!("Found mul({},{})", first, second);

        // Convert to numbers if needed
        let first_num: u32 = first.parse().unwrap();
        let second_num: u32 = second.parse().unwrap();
        sum_of_line += first_num * second_num;
        println!(" = {}", first_num * second_num);
    }
    sum_of_line
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let result = parse(
            &"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))".to_string(),
        );
        assert_eq!(result, 161);
    }
}
