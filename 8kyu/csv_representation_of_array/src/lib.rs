// https://www.codewars.com/kata/5a34af40e1ce0eb1f5000036/train/rust

use itertools::Itertools;

// First Attempt
// pub fn to_csv_text(array: &[Vec<i8>]) -> String {
//     let mut csv_text = String::new();
//     for (rows_index, rows) in array.iter().enumerate() {
//         if rows_index.gt(&0) {
//             csv_text.push_str("\n");
//         }
//         for (item_index, item) in rows.iter().enumerate() {
//             if item_index.gt(&0) {
//                 csv_text.push_str(",");
//             }
//             csv_text.push_str(item.to_string().as_str());
//         }
//     }
//     csv_text
// }

pub fn to_csv_text(array: &[Vec<i8>]) -> String {
    array
        .iter()
        .map(|row| {
            row.iter()
                .map(|number| number.to_string())
                .collect_vec()
                .join(",")
        })
        .collect_vec()
        .join("\n")
}

#[cfg(test)]
mod to_csv_text_test_suite {
    use super::to_csv_text;
    use itertools::Itertools;
    use rand::{thread_rng, Rng};

    fn do_test(input: &[Vec<i8>], expected: &str) {
        let actual = to_csv_text(input);
        assert!(
            actual == expected,
            "Test failed with array = {input:?}\nExpected \"{expected}\"\nbut got \"{actual}\""
        );
    }

    #[test]
    fn basic() {
        for (input, expected) in [
            (
                vec![
                    vec![0, 1, 2, 3, 45],
                    vec![10, 11, 12, 13, 14],
                    vec![20, 21, 22, 23, 24],
                    vec![30, 31, 32, 33, 34],
                ],
                "0,1,2,3,45\n10,11,12,13,14\n20,21,22,23,24\n30,31,32,33,34",
            ),
            (
                vec![vec![-25, 21, 2, -33, 48], vec![30, 31, -32, 33, -34]],
                "-25,21,2,-33,48\n30,31,-32,33,-34",
            ),
            (
                vec![
                    vec![5, 55, 5, 5, 55],
                    vec![6, 6, 66, 23, 24],
                    vec![127, 31, 66, 33, 7],
                ],
                "5,55,5,5,55\n6,6,66,23,24\n127,31,66,33,7",
            ),
        ] {
            do_test(&input, expected)
        }
    }

    #[test]
    fn random() {
        let mut rng = thread_rng();
        for _ in 0..100 {
            let l = rng.gen_range(2..=10_usize);
            let arr = &(0..rng.gen_range(2..=10))
                .map(|_| {
                    (0..l)
                        .map(|_| rng.gen_range(i8::MIN..=i8::MAX))
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            do_test(arr, &arr.iter().map(|row| row.iter().join(",")).join("\n"))
        }
    }
}
