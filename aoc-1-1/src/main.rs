
fn main() {

    let file = std::fs::read_to_string("input/input.txt").expect("Could not open file");
    let mut left: Vec<usize> = Vec::new();
    let mut right: Vec<usize> = Vec::new();

    file.lines().for_each(|line| {
        let mut split_whitespace = line.split_whitespace();
        let first = next_usize(&mut split_whitespace);
        left.push(first);
        let second = next_usize(&mut split_whitespace);
        right.push(second);

    });
    println!("sorted left({}) and right({})", left.len(), right.len());
    left.sort();
    right.sort();
    sum_of_distances(left, right);
    println!("Hello, world!");
}

fn next_usize( split_whitespace:  &mut std::str::SplitWhitespace<'_>) -> usize {
    split_whitespace.next().expect("no second entry in line").to_string().parse::<usize>().expect("could not parse second number")
}

fn sum_of_distances(left: Vec<usize>, right: Vec<usize>){
    let mut right_iter =right.iter();
    let mut sum = 0;
    for left_value in left {
        if let Some(right_value) = right_iter.next(){
            sum = sum + right_value.abs_diff(left_value);
        }
    }
    println!("{}", sum);

}
