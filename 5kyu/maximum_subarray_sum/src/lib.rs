// https://www.codewars.com/kata/54521e9ec8e60bc4de000d6c/train/rust

// First Attempt
// pub fn max_sequence(seq: &[i32]) -> i32 {
//     if seq.len().eq(&0) {
//         return 0;
//     }

//     let mut subarray_sum_map: Vec<Vec<i32>> = vec![Vec::with_capacity(seq.len()); seq.len()];

//     seq
//         .iter()
//         .enumerate()
//         .for_each(|(index, element)| {
//             match index {
//                 0 => {
//                     subarray_sum_map[0].push(*element);
//                     (1..seq.len()).for_each(|i| subarray_sum_map[i].push(0));
//                 },
//                 _ => (0..seq.len()).for_each(|i| {
//                     if i <= index {
//                         let last_sum = subarray_sum_map[i][index - 1];
//                         subarray_sum_map[i].push(element + last_sum);
//                     } else {
//                         subarray_sum_map[i].push(0);
//                     }
//                 })
//             };
//         });

//     let maximum_value = subarray_sum_map
//         .iter()
//         .map(|subarray| subarray.iter().max().unwrap())
//         .max()
//         .unwrap();

//     if maximum_value.lt(&0) { 0 } else { maximum_value.clone() }
// }

pub fn max_sequence(seq: &[i32]) -> i32 {
    let mut maximum = 0;

    seq.iter().fold(0, |acc, &element| {
        let local_max = element.max(acc + element);
        maximum = maximum.max(local_max);
        local_max
    });
    maximum
}

#[cfg(test)]
mod maximum_subarray_sum_test_suite {
    use super::*;
    use std::cmp::max;

    fn reference_solution(seq: &[i32]) -> i32 {
        let (mut m, mut c): (i32, i32) = (0, 0);
        for n in seq {
            c = max(0, c + n);
            m = max(c, m);
        }
        m
    }

    #[test]
    fn basic() {
        assert_eq!(max_sequence(&[-2, 1, -3, 4, -1, 2, 1, -5, 4]), 6);
        assert_eq!(max_sequence(&[11]), 11);
        assert_eq!(max_sequence(&[-32]), 0);
        assert_eq!(max_sequence(&Vec::new()), 0);
        assert_eq!(max_sequence(&[-2, 1, -7, 4, -10, 2, 1, 5, 4]), 12);
        assert_eq!(
            max_sequence(&[-83, 56, 61, -12, 39, -68, 44, -40, -47, -93]),
            144
        );
        assert_eq!(
            max_sequence(&[-32, -24, -43, -43, -55, 75, -24, 55, -49, 37]),
            106
        );
        assert_eq!(
            max_sequence(&[58, 26, 51, -88, 25, -27, -45, 86, 32, 48]),
            166
        );
    }

    #[test]
    fn random() {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        for _ in 0..100 {
            let len = rng.gen_range(0..51);
            let input: Vec<i32> = (0..len).map(|_| rng.gen_range(-100..101)).collect();
            let expected = reference_solution(&input);
            assert_eq!(
                max_sequence(&input),
                expected,
                "failed on input: {:#?}",
                input
            );
        }
    }
}
