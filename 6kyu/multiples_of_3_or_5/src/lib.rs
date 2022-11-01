// https://www.codewars.com/kata/514b92a657cdc65150000006/train/rust

pub fn solution(num: i32) -> i32 {
    (0..num).fold(0, |acc, element| {
        if element % 3 == 0 || element % 5 == 0 {
            return acc + element;
        }
        acc
    })
}

#[cfg(test)]
mod multiple_of_3_or_5_test_suite {
    use super::*;
    use rand::Rng;

    fn assertion(expected: i32, input: i32) {
        let actual = solution(input);

        assert!(
            expected == actual,
            "\nTest failed!\n expected: {}\n actual: {}\n input: {}\n",
            expected,
            actual,
            input
        );
    }

    #[test]
    fn basic() {
        assertion(23, 10);
        assertion(33, 11);
        assertion(225, 33);
        assertion(8, 6);
        assertion(3420, 123);
        assertion(543, 50);
        assertion(25719750, 10500);
    }

    #[test]
    fn random() {
        let mut rand = rand::thread_rng();
        let mut input: i32;
        let mut expected: i32;

        for _ in 0..100 {
            input = rand.gen_range(-2500..20000);

            expected = {
                let mut ans: i32 = 0;
                for x in 0..input {
                    if x % 3 == 0 || x % 5 == 0 {
                        ans = ans + x;
                    }
                }

                ans
            };

            assertion(expected, input);
        }
    }
}
