fn main() {
    println!("{}", calc("./input/day2/input.txt"));
}

fn calc(input_file: &str) -> i32 {
    let num_cubes = std::collections::HashMap::from([("red", 12), ("green", 13), ("blue", 14)]);

    let mut total_sum: i32 = (1..=100).into_iter().reduce(|acc, e| acc + e).unwrap();

    dbg!(total_sum);

    let input = std::fs::read_to_string(input_file).unwrap();

    for (id, line) in input.lines().enumerate() {
        let game = line.split_once(": ").unwrap().1;
        'line: for set in game.split("; ") {
            for draw in set.split(", ") {
                let (num, color) = draw.split_once(' ').unwrap();
                let num = str::parse::<i32>(num).unwrap();
                if num > *num_cubes.get(color).unwrap() {
                    total_sum -= id as i32 + 1;
                    break 'line;
                }
            }
        }
    }

    return total_sum;
}
