use std::collections::HashMap;

fn main() {
    calc(std::fs::read_to_string("./input/day1/input.txt").unwrap());
}

fn calc(input: String) -> i32 {
    let mut numeric_words = HashMap::<&str, usize>::new();

    numeric_words.insert("one", 1);
    numeric_words.insert("two", 2);
    numeric_words.insert("three", 3);
    numeric_words.insert("four", 4);
    numeric_words.insert("five", 5);
    numeric_words.insert("six", 6);
    numeric_words.insert("seven", 7);
    numeric_words.insert("eight", 8);
    numeric_words.insert("nine", 9);

    // let mut numeric_words = vec![
    //     "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    // ];

    let mut sum = 0;

    for (_i, line) in input.lines().enumerate() {
        let mut map = std::collections::HashMap::<usize, i32>::new();

        for (word, digit) in &numeric_words {
            if let Some(i) = line.find(word) {
                map.insert(i, *digit as i32);
            }
        }

        if let Some(i_first_digit) = line.find(|c: char| c.is_numeric()) {
            map.insert(
                i_first_digit,
                line.chars()
                    .nth(i_first_digit)
                    .unwrap()
                    .to_digit(10)
                    .unwrap() as i32,
            );
        }

        if let Some(i_last_digit) = line.rfind(|c: char| c.is_numeric()) {
            map.insert(
                i_last_digit,
                line.chars()
                    .nth(i_last_digit)
                    .unwrap()
                    .to_digit(10)
                    .unwrap() as i32,
            );
        }


        let first_num = map.get(map.keys().min().unwrap()).unwrap();
        let last_num = map.get(map.keys().max().unwrap()).unwrap();

        let num = first_num * 10 + last_num;

        sum += num;
    }
    println!("{sum}");
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_word() {
        assert_eq!(calc("one".to_string()), 11)
    }

    #[test]
    fn one_digit() {
        assert_eq!(calc("2".to_string()), 22)
    }

    #[test]
    fn one_digit_one_word() {
        assert_eq!(calc("2one".to_string()), 21)
    }

    #[test]
    fn word_digit() {
        assert_eq!(calc("one2".to_string()), 12)
    }

    #[test]
    fn digit_word_digit() {
        assert_eq!(calc("7one2".to_string()), 72)
    }

    #[test]
    fn wddw() {
        assert_eq!(calc("one27two".to_string()), 12)
    }

    #[test]
    fn w_w() {
        assert_eq!(calc("one\ntwo".to_string()), 33)
    }

    #[test]
    fn d_d() {
        assert_eq!(calc("1\n2".to_string()), 33)
    }

    #[test]
    fn smushed() {
        assert_eq!(calc("eighthree".to_string()), 83)
    }

    #[test]
    fn mid() {
        assert_eq!(calc("thoneree".to_string()), 11)
    }

}
