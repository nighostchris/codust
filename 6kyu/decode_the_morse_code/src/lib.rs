// https://www.codewars.com/kata/54b724efac3d5402db00065e/train/rust

use std::collections::HashMap;

pub fn decode_morse(encoded: &str) -> String {
    let MORSE_CODE: HashMap<String, String> = HashMap::from([
        (String::from(".-"), String::from("A")),
        (String::from("-..."), String::from("B")),
        (String::from("-.-."), String::from("C")),
        (String::from("-.."), String::from("D")),
        (String::from("."), String::from("E")),
        (String::from("..-."), String::from("F")),
        (String::from("--."), String::from("G")),
        (String::from("...."), String::from("H")),
        (String::from(".."), String::from("I")),
        (String::from(".---"), String::from("J")),
        (String::from("-.-"), String::from("K")),
        (String::from(".-.."), String::from("L")),
        (String::from("--"), String::from("M")),
        (String::from("-."), String::from("N")),
        (String::from("---"), String::from("O")),
        (String::from(".--."), String::from("P")),
        (String::from("--.-"), String::from("Q")),
        (String::from(".-."), String::from("R")),
        (String::from("..."), String::from("S")),
        (String::from("-"), String::from("T")),
        (String::from("..-"), String::from("U")),
        (String::from("...-"), String::from("V")),
        (String::from(".--"), String::from("W")),
        (String::from("-..-"), String::from("X")),
        (String::from("-.--"), String::from("Y")),
        (String::from("--.."), String::from("Z")),
    ]);

    encoded
        .split(" ")
        .map(|code| match code {
            "" => String::from(" "),
            _ => MORSE_CODE.get(code).unwrap().to_string(),
        })
        .collect::<Vec<String>>()
        .join("")
        .split_whitespace()
        .collect::<Vec<&str>>()
        .join(" ")
}

#[cfg(test)]
mod decode_morse_test_suite {
    use super::*;

    #[test]
    fn basic() {
        assert_eq!(decode_morse(".... . -.--   .--- ..- -.. ."), "HEY JUDE");
        assert_eq!(decode_morse(""), "");
        assert_eq!(decode_morse(".-"), "A");
        assert_eq!(decode_morse("."), "E");
        assert_eq!(decode_morse(".."), "I");
        assert_eq!(decode_morse(". ."), "EE");
        assert_eq!(decode_morse(".   ."), "E E");
        assert_eq!(decode_morse("... --- ..."), "SOS");
        assert_eq!(decode_morse("...   ---   ..."), "S O S");
        assert_eq!(decode_morse("   "), "");
        assert_eq!(decode_morse(" . "), "E");
        assert_eq!(decode_morse("   .   . "), "E E");
        assert_eq!(decode_morse(".   .   "), "E E");
    }
}
