fn main() {
    println!("{}", calc("./input/day2/input.txt"));
}

fn calc(input_file: &str) -> i32 {
    let mut sum = 0;

    let input = std::fs::read_to_string(input_file).unwrap();

    for (_id, line) in input.lines().enumerate() {
        let mut min_num_cubes =
            std::collections::HashMap::from([("red", 0), ("green", 0), ("blue", 0)]);

        let game = line.split_once(": ").unwrap().1;

        for set in game.split("; ") {
            for draw in set.split(", ") {
                let (num, color) = draw.split_once(' ').unwrap();
                let num = str::parse::<i32>(num).unwrap();
                if num > *min_num_cubes.get(color).unwrap() {
                    min_num_cubes.insert(color, num);
                }
            }
        }
        // In theory, we don't even need the unwrap_or here since the reduced value should be zero
        // even if no "sets" were drawn
        sum += min_num_cubes.into_values().reduce(|acc, e| acc * e).unwrap_or(0);
    }

    return sum;
}
