fn main() {
    let file = std::fs::read_to_string("input/input.txt").expect("Unable to read file");
    let lines: Vec<&str> = file.lines().collect();
    let count = from_lines(lines);
    println!("Safe Records: {}", count);
}

fn from_lines(lines: Vec<&str>) -> usize {
    let count = lines.iter().filter(|e| is_safe(e)).count();
    count
}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    None,
}

impl Direction {
    fn from_diff(diff: isize) -> Direction {
        match diff {
            ..0 => Direction::Up,
            0 => Direction::None,
            1.. => Direction::Down,
        }
    }
}
#[derive(Debug)]
struct Toleration {
    toleration_count: u32,
    reason_1: Option<String>,
    reason_2: Option<String>,
}
impl Toleration {
    fn new() -> Toleration {
        Toleration {
            toleration_count: 1,
            reason_1: None,
            reason_2: None,
        }
    }
    fn toleration_limit_reached(&mut self, reason: &str) -> bool {
        if self.toleration_count == 0 {
            self.reason_2 = Some(reason.to_string());
            return true;
        } else {
            self.toleration_count = self.toleration_count - 1;
            self.reason_1 = Some(reason.to_string());
            return false;
        }
    }
}

fn is_safe(line: &str) -> bool {
    let numbers: Vec<isize> = line
        .split_whitespace()
        .map(|n| n.parse::<isize>().unwrap())
        .collect();
    let mut last_number = None;
    let mut direction = None;
    let mut toleration = Toleration::new();
    for number in numbers.clone() {
        if let Some(last) = last_number {
            let difference: isize = last - number;
            if !(1..=3).contains(&difference.abs()) {
                if toleration.toleration_limit_reached("Distance") {
                    println!(
                        "Unsafe: {} {:?} ",
                        numbers
                            .iter()
                            .map(|each| each.to_string())
                            .collect::<Vec<String>>()
                            .join(" "),
                        toleration
                    );
                    return false;
                } else {
                    continue;
                }
            }
            if direction.is_none() {
                direction = Some(Direction::from_diff(difference));
            } else {
                let current_direction = Direction::from_diff(difference);
                if current_direction != direction.unwrap() {
                    if toleration.toleration_limit_reached("Direction") {
                        return false;
                    } else {
                        continue;
                    }
                }
            }
        }
        last_number = Some(number);
    }
    println!(
        "Safe: {:?} {} {:?} ",
        direction.unwrap(),
        numbers
            .iter()
            .map(|each| each.to_string())
            .collect::<Vec<String>>()
            .join(" "),
        toleration
    );

    true
}

#[cfg(test)]
pub mod test {
    use crate::{from_lines, is_safe};

    #[test]
    fn test_data() {
        let input = r#"7 6 4 2 1
         1 2 7 8 9
         9 7 6 2 1
         1 3 2 4 5
         8 6 4 4 1
         1 3 6 7 9"#;

        let lines = input.lines().collect::<Vec<&str>>();
        let count = from_lines(lines);

        assert_eq!(count, 4);
    }

    use rstest::rstest;

    #[rstest]
    #[case(r#"1 2 3 4 5 6"#, true)]
    #[case(r#"1 2 3 4 5 5"#, true)]
    #[case(r#"1 1 3 4 5 6"#, true)]
    #[case(r#"1 2 0 4 5 6"#, true)]
    #[case(r#"1 5 3 7 8 6"#, false)]
    #[case(r#"1 5 9 10 11 12"#, false)]
    #[case(r#"1 2 3 4 5 6"#, true)]
    fn is_safe_test(#[case] input: &str, #[case] expected: bool) {
        assert_eq!(expected, fibonacci(input))
    }

    fn fibonacci(input: &str) -> bool {
        is_safe(input)
    }
}
