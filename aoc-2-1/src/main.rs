fn main() {

    let file = std::fs::read_to_string("input/input.txt").expect("Unable to read file");
    let lines: Vec<&str> = file.lines().collect();
    let count = lines.iter().filter(|e|is_safe(e)).count();
    println!("Safe Records: {}", count);

}

#[derive(PartialEq, Clone, Copy, Debug)]
enum Direction {
    Up,
    Down,
    None
}

impl Direction {
    fn from_diff(diff: isize) -> Direction {
        match diff {
            ..0 => Direction::Down,
            0 => Direction::None,
            1.. => Direction::Up
            }

    }
}



fn is_safe(line: &str) -> bool {
    let numbers: Vec<isize> = line.split_whitespace().map(|n|n.parse::<isize>().unwrap()).collect();
    let mut last_number = None;
    let mut direction = None;
    let mut tolerated = false;
    for  number in numbers.clone(){
        if let Some(last) = last_number {

            let difference: isize = last - number;
            if !(1..=3).contains(&difference.abs()){
                match tolerated {
                    false =>
                        tolerated = true,
                        true => return false

            }
            if direction.is_none() {
                direction = Some(Direction::from_diff(difference));
            }else {
                let current_direction = Direction::from_diff(difference);
                if current_direction != direction.unwrap() {
                    return false;
                }
            }
        }
        last_number = Some(number);


    }
    println!("Found Safe: {:?} {} ",direction, numbers.iter().map(|each|each.to_string()).collect::<Vec<String>>().join(" "));

    true
}
