// https://www.codewars.com/kata/556deca17c58da83c00002db/train/rust

pub fn tribonacci(signature: &[f64; 3], n: usize) -> Vec<f64> {
    let sequence = signature.to_vec();

    match n {
        0 => vec![],
        1 => sequence[0..1].to_vec(),
        2 => sequence[0..2].to_vec(),
        3 => sequence[0..3].to_vec(),
        _ => (0..n - 3).fold(sequence, |mut acc, index| {
            acc.push(acc[0 + index..3 + index].iter().sum());
            acc
        }),
    }
}

#[cfg(test)]
mod tribonacci_test_suite {
    use super::*;
    extern crate rand;
    use self::rand::Rng;

    const EPS: f64 = 1e-6;

    fn double_abs_diff(a: f64, b: f64) -> f64 {
        (a - b).abs()
    }

    fn double_rel_diff(a: f64, b: f64) -> f64 {
        ((a - b) / b).abs()
    }

    fn assert_double_slice_eq(a: &[f64], b: &[f64], err: &str) {
        assert!(a.len() == b.len(), "{}: expected {:?}, got {:?}", err, b, a);
        for (&l, &r) in a.iter().zip(b.iter()) {
            assert!(
                double_abs_diff(l, r) < EPS || double_rel_diff(l, r) < EPS,
                "{}: expected {:?}, got {:?}",
                err,
                b,
                a
            );
        }
    }

    fn test_tribonacci(signature: &[f64; 3], n: usize, expected: &[f64]) {
        assert_double_slice_eq(
            &tribonacci(signature, n),
            expected,
            &format!("tribonacci({:?}, {})", signature, n),
        );
    }

    fn soluzionacci(signature: &[f64; 3], n: usize) -> Vec<f64> {
        let mut res = vec![];
        for i in 0..n {
            if i < 3 {
                res.push(signature[i]);
            } else {
                let n = res.len();
                let t = res[n - 1] + res[n - 2] + res[n - 3];
                res.push(t);
            }
        }
        return res;
    }

    #[test]
    fn basic() {
        assert_eq!(
            tribonacci(&[0., 1., 1.], 10),
            vec![0., 1., 1., 2., 4., 7., 13., 24., 44., 81.]
        );
        assert_eq!(
            tribonacci(&[1., 0., 0.], 10),
            vec![1., 0., 0., 1., 1., 2., 4., 7., 13., 24.]
        );
        assert_eq!(
            tribonacci(&[0., 0., 0.], 10),
            vec![0., 0., 0., 0., 0., 0., 0., 0., 0., 0.]
        );
        assert_eq!(
            tribonacci(&[1., 2., 3.], 10),
            vec![1., 2., 3., 6., 11., 20., 37., 68., 125., 230.]
        );
        assert_eq!(
            tribonacci(&[3., 2., 1.], 10),
            vec![3., 2., 1., 6., 9., 16., 31., 56., 103., 190.]
        );
        assert_eq!(tribonacci(&[1., 1., 1.], 1), vec![1.]);
        assert_eq!(tribonacci(&[300., 200., 100.], 0), vec![]);
        assert_eq!(
            tribonacci(&[0.5, 0.5, 0.5], 30),
            vec![
                0.5, 0.5, 0.5, 1.5, 2.5, 4.5, 8.5, 15.5, 28.5, 52.5, 96.5, 177.5, 326.5, 600.5,
                1104.5, 2031.5, 3736.5, 6872.5, 12640.5, 23249.5, 42762.5, 78652.5, 144664.5,
                266079.5, 489396.5, 900140.5, 1655616.5, 3045153.5, 5600910.5, 10301680.5
            ]
        );
    }

    #[test]
    fn random() {
        let mut rng = rand::thread_rng();
        for _ in 0..40 {
            let sign = [
                rng.gen_range(0.0..20.0),
                rng.gen_range(0.0..20.0),
                rng.gen_range(0.0..20.0),
            ];
            let n = rng.gen_range(0..50);
            test_tribonacci(&sign, n, &soluzionacci(&sign, n)[..]);
        }
    }
}
