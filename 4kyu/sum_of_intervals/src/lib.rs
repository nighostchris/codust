// https://www.codewars.com/kata/52b7ed099cdc285c300001cd/train/rust

pub fn sum_intervals(intervals: &[(i32, i32)]) -> i32 {
    let mut sorted_intervals: Vec<(i32, i32)> = intervals.to_vec();
    let mut merged_intervals: Vec<(i32, i32)> = Vec::new();

    sorted_intervals.sort_by(|a, b| a.0.cmp(&b.0));

    sorted_intervals
        .iter()
        .for_each(|&interval| match merged_intervals.last_mut() {
            Some(last_interval) => {
                if (interval.0.lt(&last_interval.1) && interval.1.gt(&last_interval.1))
                    || interval.0.eq(&last_interval.1)
                {
                    last_interval.1 = interval.1;
                } else if interval.0.gt(&last_interval.1) {
                    merged_intervals.push(interval);
                }
            }
            None => merged_intervals.push(interval),
        });

    merged_intervals
        .iter()
        .map(|interval| interval.1 - interval.0)
        .sum()
}

#[cfg(test)]
mod sum_intervals_test_suite {
    use super::*;
    use itertools::Itertools;
    use rand::{thread_rng, Rng};

    const ERR_MSG: &str = "\nYour result (left) did not match expected output (right).";

    fn ref_solution(intervals: &[(i32, i32)]) -> i32 {
        let mut sum = 0;
        let mut end = i32::MIN;
        for (a, b) in intervals.iter().sorted() {
            if end < *a {
                end = *a;
            }
            if end < *b {
                sum += b - end;
                end = *b;
            }
        }
        sum
    }

    #[test]
    fn basic() {
        let mut input = vec![(1, 6)];
        assert_eq!(sum_intervals(&input), 5, "{}{:?}", ERR_MSG, &input);
        input = vec![(1, 4), (6, 10)];
        assert_eq!(sum_intervals(&input), 7, "{}{:?}", ERR_MSG, &input);
        input = vec![(11, 15), (6, 10), (1, 2)];
        assert_eq!(sum_intervals(&input), 9, "{}{:?}", ERR_MSG, &input);
        let mut input = vec![(1, 5), (1, 5)];
        assert_eq!(sum_intervals(&input), 4, "{}{:?}", ERR_MSG, &input);
        input = vec![(1, 5), (5, 10)];
        assert_eq!(sum_intervals(&input), 9, "{}{:?}", ERR_MSG, &input);
        input = vec![(1, 4), (3, 6), (5, 8), (7, 10), (9, 12)];
        assert_eq!(sum_intervals(&input), 11, "{}{:?}", ERR_MSG, &input);
        input = vec![(1, 4), (7, 10), (3, 5)];
        assert_eq!(sum_intervals(&input), 7, "{}{:?}", ERR_MSG, &input);
        input = vec![(1, 20), (2, 19), (5, 15), (8, 12)];
        assert_eq!(sum_intervals(&input), 19, "{}{:?}", ERR_MSG, &input);
        input = vec![(1, 5), (10, 20), (1, 6), (16, 19), (5, 11)];
        assert_eq!(sum_intervals(&input), 19, "{}{:?}", ERR_MSG, &input);
        input = vec![(2, 3), (2, 6), (2, 4), (2, 9), (2, 5)];
        assert_eq!(sum_intervals(&input), 7, "{}{:?}", ERR_MSG, &input);
        let mut input = vec![(-1_000_000_000, 1_000_000_000)];
        assert_eq!(
            sum_intervals(&input),
            2_000_000_000,
            "{}{:?}",
            ERR_MSG,
            &input
        );
        input = vec![(0, 20), (-100_000_000, 10), (30, 40)];
        assert_eq!(
            sum_intervals(&input),
            100_000_030,
            "{}{:?}",
            ERR_MSG,
            &input
        );
    }

    #[test]
    fn random() {
        let mut rng = thread_rng();

        for _ in 0..250 {
            let intervals: Vec<(i32, i32)> = (0..rng.gen_range(1..50))
                .map(|_| {
                    let start: i32 = rng.gen_range(-100..99);
                    let end: i32 = rng.gen_range((start + 1)..100);
                    (start, end)
                })
                .collect();

            assert_eq!(
                sum_intervals(&intervals),
                ref_solution(&intervals),
                "\nYour result (left) did not match the expected output (right) for this input:\n{:?}",
                &intervals,
            );
        }

        for _ in 0..100 {
            let intervals: Vec<(i32, i32)> = (0..rng.gen_range(1000..10000))
                .map(|_| {
                    let start: i32 = rng.gen_range(-1_000_000_000..1_000_000_000 - 10_000_000);
                    let width: i32 = rng.gen_range(1_000_000..10_000_000);
                    (start, start + width)
                })
                .collect();

            assert_eq!(
                sum_intervals(&intervals),
                ref_solution(&intervals),
                "\nYour result (left) did not match the expected output (right) for this input:\n{:?}",
                &intervals,
            );
        }
    }
}
