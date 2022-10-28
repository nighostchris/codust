// https://www.codewars.com/kata/51e0007c1f9378fa810002a9/train/rust

// First Attempt
// pub fn parse(code: &str) -> Vec<i32> {
//     let mut value = 0;
//     let mut output = Vec::new();

//     for command in code.chars() {
//         if (command == 'i') {
//             value += 1;
//             continue;
//         }

//         if (command == 'd') {
//             value -= 1;
//             continue;
//         }

//         if (command == 's') {
//             value *= value;
//             continue;
//         }

//         if (command == 'o') {
//             output.push(value);
//             continue;
//         }
//     }
//     output
// }

pub fn parse(code: &str) -> Vec<i32> {
    let mut value = 0;
    let mut output = Vec::new();

    for command in code.chars() {
        match command {
            'i' => value += 1,
            'd' => value -= 1,
            's' => value *= value,
            'o' => output.push(value),
            _ => {}
        }
    }
    output
}

#[cfg(test)]
mod parse_test_suite {
    extern crate rand;

    use super::*;
    use rand::Rng;

    #[test]
    fn basic() {
        assert_eq!(parse("iiisdoso"), vec![8, 64]);
        assert_eq!(parse("iiisdosodddddiso"), vec![8, 64, 3600]);
    }

    fn solution(code: &str) -> Vec<i32> {
        let mut output = Vec::new();
        let mut value = 0;

        for c in code.chars() {
            match c {
                'i' => {
                    value += 1;
                }
                'd' => {
                    value -= 1;
                }
                's' => {
                    value *= value;
                }
                'o' => {
                    output.push(value);
                }
                _ => {
                    continue;
                }
            };
        }

        return output;
    }

    fn random_deadfish() -> String {
        let chars: Vec<char> = "idsoa".chars().collect();
        let mut fish = String::new();

        for _ in 0..6 {
            fish.push(chars[rand::thread_rng().gen_range(0..chars.len())]);
        }
        fish.push('o');

        return fish;
    }

    #[test]
    fn random() {
        for _ in 0..1000 {
            let fish: &str = &random_deadfish();

            assert_eq!(parse(fish), solution(fish));
        }
    }
}
