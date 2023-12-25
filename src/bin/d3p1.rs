use regex::Regex;

fn main() {
    let input = std::fs::read_to_string("./input/day3/input.txt").unwrap();
    println!("{}", calc(&input));
    println!(
        "{}",
        calc(
            "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
"
        )
    );
}

fn calc(input_str: &str) -> i32 {
    let re_dig = Regex::new(r"\d+").unwrap();
    let re_sym = Regex::new(r"[^\d\.]").unwrap();

    let mut prev_line: Option<&str> = None;
    let mut cur_line: Option<&str> = None;
    let mut next_line: Option<&str> = None;

    let mut lines = input_str.lines();

    let mut sum = 0i32;

    loop {
        prev_line = cur_line;
        cur_line = next_line;

        if let Some(line) = lines.next() {
            next_line = Some(line);
        } else {
            next_line = None;
        }

        if let Some(line) = cur_line {
            let dig_matches: Vec<regex::Match> = re_dig.find_iter(line).collect();
            let sym_matches: Vec<regex::Match> = re_sym.find_iter(line).collect();

            compare_matches(&dig_matches, &sym_matches, &mut sum);

            for surrounding_line in [prev_line, next_line] {
                if let Some(line) = surrounding_line {
                    let sym_matches: Vec<regex::Match> = re_sym.find_iter(line).collect();
                    compare_matches(&dig_matches, &sym_matches, &mut sum)
                }
            }
        } else {
            continue;
        }

        if next_line == None {
            break;
        }
    }

    return sum;
}

fn compare_matches(
    dig_matches: &Vec<regex::Match>,
    sym_matches: &Vec<regex::Match>,
    sum: &mut i32,
) {
    for dig_match in dig_matches {
        for sym_match in sym_matches {
            let i_sym = sym_match.start();
            let mut start = dig_match.start();
            let end = dig_match.end();
            let num = dig_match.as_str().parse::<i32>().unwrap();

            if start != 0 {
                start -= 1;
            }

            if i_sym >= start && i_sym <= end {
                // dbg!(i_sym, start, end, num, sym_match.as_str());
                *sum += num
            }
        }
    }
}
