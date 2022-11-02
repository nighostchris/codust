// https://www.codewars.com/kata/521c2db8ddc89b9b7a0000c1/train/rust

pub fn snail(matrix: &[Vec<i32>]) -> Vec<i32> {
    let mut vec_matrix = matrix.to_vec();
    let mut output: Vec<i32> = vec![];

    while vec_matrix.len().gt(&0) {
        // Get and remove the first row
        let row = vec_matrix.remove(0);
        output = [output, row].concat();

        // Pop the last item of rows that are not first and last
        (0..vec_matrix.len()).for_each(|index| match vec_matrix[index].pop() {
            Some(element) => output.push(element),
            None => {}
        });

        // Pop the last row
        match vec_matrix.pop() {
            Some(mut last_row) => {
                last_row.reverse();
                output = [output, last_row].concat();
            }
            None => {}
        }

        // Get all the first item from rows that are not first + 1 and last
        (0..vec_matrix.len()).rev().for_each(|index| {
            let first_element = vec_matrix[index].remove(0);
            output.push(first_element);
        });
    }
    output
}

#[cfg(test)]
mod snail_test_suite {
    use super::*;

    fn do_test(mat: &[Vec<i32>], exp: &[i32]) {
        assert_eq!(snail(mat), exp, "\nFailed with input {:?}", mat);
    }

    fn reference_solution(matrix: &[Vec<i32>]) -> Vec<i32> {
        if matrix.len() == 0 {
            return Vec::new();
        } else if matrix.len() == 1 {
            return matrix[0].iter().rev().cloned().collect();
        }

        let mut res: Vec<i32> = matrix[0].clone();
        let matrix: Vec<Vec<i32>> = matrix.iter().skip(1).cloned().collect();
        res.append(&mut reference_solution(&reference_helper(&matrix)));
        res
    }

    fn reference_helper(matrix: &[Vec<i32>]) -> Vec<Vec<i32>> {
        let rows = matrix.len();
        if rows < 2 {
            return matrix.iter().cloned().collect();
        }
        let cols = matrix[0].len();

        let mut res = vec![vec![0; rows]; cols];
        for r in 0..rows {
            for c in 0..cols {
                res[c][r] = matrix[r][cols - c - 1];
            }
        }

        res
    }

    #[test]
    fn basic() {
        let square = &[vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]];
        let expected = vec![1, 2, 3, 6, 9, 8, 7, 4, 5];
        do_test(square, &expected);

        let square = &[vec![1, 2, 3], vec![8, 9, 4], vec![7, 6, 5]];
        let expected = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        do_test(square, &expected);

        let square: &[Vec<i32>; 1] = &[Vec::new()];
        let expected = Vec::new();
        assert_eq!(snail(square), expected, "Failed with empty input");

        let square = &[vec![1]];
        let expected = vec![1];
        do_test(square, &expected);

        let square = &[
            vec![1, 2, 3, 4, 5],
            vec![6, 7, 8, 9, 10],
            vec![11, 12, 13, 14, 15],
            vec![16, 17, 18, 19, 20],
            vec![21, 22, 23, 24, 25],
        ];
        let expected = vec![
            1, 2, 3, 4, 5, 10, 15, 20, 25, 24, 23, 22, 21, 16, 11, 6, 7, 8, 9, 14, 19, 18, 17, 12,
            13,
        ];
        do_test(square, &expected);

        let square = &[
            vec![1, 2, 3, 4, 5, 6],
            vec![20, 21, 22, 23, 24, 7],
            vec![19, 32, 33, 34, 25, 8],
            vec![18, 31, 36, 35, 26, 9],
            vec![17, 30, 29, 28, 27, 10],
            vec![16, 15, 14, 13, 12, 11],
        ];
        let expected = vec![
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36,
        ];
        do_test(square, &expected);
    }

    #[test]
    fn random() {
        use rand::Rng;
        let mut rng = rand::thread_rng();

        // forcefully test sizes 0..20
        for n in 0..=20 {
            let sq: Vec<Vec<i32>> = (0..n)
                .map(|_| (0..n).map(|_| rng.gen_range(0..=1_000)).collect())
                .collect();
            let expected = reference_solution(&sq);
            do_test(&sq, &expected);
        }

        for _ in 1..80 {
            let n = rng.gen_range(1..=20);
            let sq: Vec<Vec<i32>> = (0..n)
                .map(|_| (0..n).map(|_| rng.gen_range(0..=1_000)).collect())
                .collect();
            let expected = reference_solution(&sq);
            do_test(&sq, &expected);
        }
    }
}
