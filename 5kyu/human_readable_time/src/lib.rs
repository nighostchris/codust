// https://www.codewars.com/kata/52685f7382004e774f0001f7/train/rust

// First Attempt
// pub fn make_readable(seconds: u32) -> String {
//     let mut total_seconds: f32 = seconds as f32;
//     let mut time = String::new();

//     if total_seconds.ge(&3600.0) {
//         let no_of_hours = (total_seconds / 3600.0).floor();
//         total_seconds -= no_of_hours * 3600.0;
//         if no_of_hours.lt(&10.0) {
//             time.push_str(format!("0{}:", no_of_hours).as_str());
//         } else {
//             time.push_str(format!("{}:", no_of_hours).as_str());
//         }
//     } else {
//         time.push_str("00:");
//     }

//     if total_seconds.ge(&60.0) {
//         let no_of_seconds = (total_seconds / 60.0).floor();
//         total_seconds -= no_of_seconds * 60.0;
//         if no_of_seconds.lt(&10.0) {
//             time.push_str(format!("0{}:", no_of_seconds).as_str());
//         } else {
//             time.push_str(format!("{}:", no_of_seconds).as_str());
//         }
//     } else {
//         time.push_str("00:");
//     }

//     if total_seconds.lt(&10.0) {
//         time.push_str(format!("0{}", total_seconds).as_str());
//     } else {
//         time.push_str(format!("{}", total_seconds).as_str());
//     }

//     time
// }

pub fn make_readable(seconds: u32) -> String {
    let s = seconds % 60;
    let mut m = seconds / 60;
    let h = m / 60;
    m = m % 60;

    format!("{:02}:{:02}:{:02}", h, m, s)
}

#[cfg(test)]
mod human_readable_time_test_suite {
    use super::*;
    use rand::{thread_rng, Rng};

    const ERR_MSG: &str = "\nYour result (left) did not match the expected output (right)";

    fn reference_solution(se: u32) -> String {
        let h = se / 3600;
        let m = (se % 3600) / 60;
        let s = se - 3600 * h - 60 * m;
        format!("{h:0>2}:{m:0>2}:{s:0>2}")
    }

    fn dotest(s: u32, expected: &str) {
        assert_eq!(make_readable(s), expected, "{ERR_MSG} with seconds = {s}")
    }

    #[test]
    fn basic() {
        dotest(0, "00:00:00");
        dotest(59, "00:00:59");
        dotest(60, "00:01:00");
        dotest(3599, "00:59:59");
        dotest(3600, "01:00:00");
        dotest(86399, "23:59:59");
        dotest(86400, "24:00:00");
        dotest(359999, "99:59:59");
    }

    #[test]
    fn random() {
        let mut rng = thread_rng();
        for _ in 0..100 {
            let s = rng.gen_range(0..360_000);
            dotest(s, &reference_solution(s));
        }
    }
}
