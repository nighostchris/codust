// https://www.codewars.com/kata/5426d7a2c2c7784365000783/train/rust

fn generate_balanced_parentheses(
    open_bracket: u16,
    close_bracket: u16,
    cache: String,
    output: &mut Vec<String>,
) {
    // Ensuring always consume all available open brackets first
    if open_bracket.gt(&close_bracket) {
        return;
    }
    // End recursion when reaching the tree bottom
    if open_bracket.eq(&0) && close_bracket.eq(&0) {
        output.push(cache);
    } else if close_bracket.eq(&open_bracket) {
        generate_balanced_parentheses(open_bracket - 1, close_bracket, format!("{}(", cache), output);
    } else if close_bracket.gt(&open_bracket) {
        generate_balanced_parentheses(open_bracket, close_bracket - 1, format!("{})", cache), output);
        if open_bracket.ge(&1) {
            generate_balanced_parentheses(open_bracket - 1, close_bracket, format!("{}(", cache), output);
        }
    }
}

pub fn balanced_parens(n: u16) -> Vec<String> {
    let mut output: Vec<String> = Vec::new();
    generate_balanced_parentheses(n, n, String::new(), &mut output);
    output
}

#[cfg(test)]
mod balanced_parens_test_suite {
    use super::*;
    use rand::{thread_rng};
    use rand::seq::SliceRandom;
    use std::collections::HashSet;

    fn reference_solution(n: u16) -> Vec<String> {
        match n {
            0 => vec![String::from("")],
            1 => vec![String::from("()")],
            _ => {
                    let rec_res = balanced_parens(n - 1);
                    let mut h = HashSet::new();
                    rec_res
                    .iter()
                    .for_each(|s|
                    {
                        let mut o = 0;
                        h.extend([
                            ["()", s].join(""), 
                            [s, "()"].join(""), 
                            ["(", s, ")"].join(""),
                        ]);
                        s
                        .chars()
                        .enumerate()
                        .for_each(|(i, c)|
                        {
                            if o == 0 {
                                h.extend([
                                    ["(", &s[..i], ")", &s[i..]].join(""),
                                    [&s[..i], "()", &s[i..]].join(""),
                                ]);
                            }
                            o += if c == '(' {1} else {-1}; 
                            });
                    });
                    Vec::from_iter(h)
            }
        }
    }

    fn do_test(n: u16, expected: Vec<String>) {
        let mut actual = balanced_parens(n);
        actual.sort();
        assert!(
            actual == expected,
            "With n = {n}\nExpected \"{expected:?}\"\nBut got \"{actual:?}\""
        );
    }

    #[test]
    fn basic() {
        let tests = [
            (0, vec![String::from("")]),
            (1, vec![String::from("()")]),
            (2, vec![String::from("(())"), String::from("()()")]),
            (3, vec![String::from("((()))"), String::from("(()())"), String::from("(())()"), String::from("()(())"), String::from("()()()")]),
        ];
        for (n, exp) in tests.iter() {
            do_test(*n, exp.to_vec())
        }
    }

    #[test]
    fn random() {
        let mut rng = thread_rng();
        let mut xs = (0..14).collect::<Vec<u16>>();
        xs.shuffle(&mut rng);
        for n in xs {
            let mut expected = reference_solution(n);
            expected.sort();
            do_test(n, expected);
        }
    }
}
