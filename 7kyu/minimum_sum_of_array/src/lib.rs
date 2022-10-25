// https://www.codewars.com/kata/5a523566b3bfa84c2e00010b/train/rust

// First Attempt
// pub fn min_sum(xs: &[u64]) -> u64 {
//     let mut list: Vec<u64> = xs.to_vec();
//     list.sort();
//     let mut sum = 0;
//     let pivot = list.len() / 2;
//     for index in 0..pivot {
//         sum += list[index] * list[list.len() - index - 1];
//     }
//     sum
// }

pub fn min_sum(xs: &[u64]) -> u64 {
    let mut list = xs.to_vec();
    list.sort();

    (0..list.len() / 2)
        .map(|index| list[index] * list[list.len() - index - 1])
        .sum()
}

#[cfg(test)]
mod min_sum_test_suite {
    use super::*;
    use rand::Rng;

    #[test]
    fn basic() {
        assert_eq!(min_sum(&[5, 4, 2, 3]), 22);
        assert_eq!(min_sum(&[12, 6, 10, 26, 3, 24]), 342);
        assert_eq!(min_sum(&[9, 2, 8, 7, 5, 4, 0, 6]), 74);
    }

    #[test]
    fn random() {
        let mut rng = rand::thread_rng();

        for _ in 0..100 {
            // By multiplying by 2 the generated array lengths will always be even
            let xs = (0..2 * rng.gen_range(2..102))
                .map(|_| rng.gen_range(0..201))
                .collect::<Vec<_>>();

            assert_eq!(min_sum(&xs[..]), min_sum_solution(&xs[..]));
        }
    }

    fn min_sum_solution(xs: &[u64]) -> u64 {
        let mut xs = xs.to_vec();
        xs.sort_unstable();

        (0..xs.len() / 2).fold(0, |sum, i| sum + xs[i] * xs[xs.len() - 1 - i])
    }
}
