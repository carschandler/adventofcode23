fn main() {
    let input = std::fs::read_to_string("./input/day1/input.txt").unwrap();
    let mut sum = 0;
    for line in input.lines() {
        let first = line
            .chars()
            .nth(line.find(|c: char| c.is_numeric()).unwrap())
            .unwrap();
        let last = line
            .chars()
            .nth(line.rfind(|c: char| c.is_numeric()).unwrap())
            .unwrap();
        let numstr = first.to_string() + &last.to_string();
        let num: i32 = str::parse(&numstr).unwrap();
        sum += num;
    }
    println!("Sum: {sum}");
}
