// https://www.codewars.com/kata/59d727d40e8c9dd2dd00009f/train/rust

use regex::Regex;

// First Attempt
// pub fn balance(book: &str) -> String {
//     let re = Regex::new(r"^[A-Za-z0-9\.\s]$").unwrap();

//     let raw_book: Vec<String> = book
//         .split("\n")
//         .filter(|record| record.len() != 0)
//         .map(|record| {
//             record
//                 .chars()
//                 .filter(|&character| re.is_match(character.to_string().as_str()))
//                 .collect::<String>()
//         })
//         .collect();

//     let mut spent: f32 = 0.0;
//     let mut balance = raw_book.get(0).unwrap().parse::<f32>().unwrap();

//     let mut final_book = raw_book
//         .iter()
//         .enumerate()
//         .map(|(key, value)| match key {
//             0 => format!("Original Balance: {:.2}", value.parse::<f32>().unwrap()),
//             _ => {
//                 let split: Vec<&str> = value.split(" ").collect();
//                 let amount = split.get(2).unwrap().parse::<f32>().unwrap();
//                 balance -= amount;
//                 spent += amount;
//                 format!(
//                     "{} {} {:.2} Balance {:.2}",
//                     split.get(0).unwrap(),
//                     split.get(1).unwrap(),
//                     amount,
//                     balance
//                 )
//             }
//         })
//         .collect::<Vec<String>>()
//         .join("\n");

//     final_book.push_str(format!("\nTotal expense  {:.2}", spent).as_str());
//     let average_expense = spent / (raw_book.len() - 1) as f32;
//     let truncated_average_expense = (average_expense * 1000.0).trunc() / 1000.0;
//     final_book.push_str(format!("\nAverage expense  {:.2}", truncated_average_expense).as_str());

//     final_book
// }

pub fn balance(book: &str) -> String {
    let re = Regex::new(r"[^\w\d\.\s]").unwrap();

    let book = re.replace_all(book, "");
    let book: Vec<&str> = book.split("\n").filter(|x| !x.is_empty()).collect();

    let mut spent: f32 = 0.0;
    let mut balance = book[0].parse::<f32>().unwrap();

    let mut output = book
        .iter()
        .enumerate()
        .map(|(key, value)| match key {
            0 => format!("Original Balance: {:.2}", value.parse::<f32>().unwrap()),
            _ => {
                let split: Vec<&str> = value.split(" ").collect();
                let (check_number, category, amount) = (split[0], split[1], split[2]);
                let amount = amount.parse::<f32>().unwrap();
                balance -= amount;
                spent += amount;
                format!(
                    "{} {} {:.2} Balance {:.2}",
                    check_number, category, amount, balance
                )
            }
        })
        .collect::<Vec<String>>()
        .join("\n");

    output.push_str(&format!("\nTotal expense  {:.2}", spent));
    output.push_str(&format!(
        "\nAverage expense  {:.2}",
        spent / (book.len() - 1) as f32
    ));

    output
}

#[cfg(test)]
mod balance_test_suite {
    extern crate regex as oregex;
    use super::*;

    fn dotest(book: &str, exp: &str) -> () {
        println!("book:{}", book);
        let ans = balance(book);
        println!("actual:\n{}", ans);
        println!("expect:\n{}", exp);
        println!("{}", ans == exp);
        assert_eq!(ans, exp);
        println!("{}", "-");
    }

    extern crate rand;
    use self::rand::Rng;

    fn compose(k: i32) -> String {
        let mut rng = rand::thread_rng();
        let ctgr = vec![
            "Market",
            "Hardware",
            "Video",
            "Books",
            "Music",
            "Gasoline",
            "Beauty",
            "Perfume",
            "Pen",
            "Grocery",
            "Stamps",
            "Photos",
            "Picture",
            "Vegetables",
            "Flowers",
            "Fruits",
            "Hairdresser",
            "Meat",
            "Car",
            "Tyres",
        ]; // 20
        let prices = vec![
            "120.3", "13.1", "17.6", "93.5", "3.2", "71.4", "12.2", "120.90", "34.00", "13.6",
            "11.00", "12.00", "110.7", "24.8", "54.00", "17.5", "128.00", "17.00", "19.00,",
            "20.00",
        ]; // 20
        let chks = vec![
            "001", "002", "003", "120", "121", "122", "123", "124", "125", "126", "127", "128",
            "129", "130", "131", "131", "132", "129", "139", "160",
        ];
        let bal = (rng.gen_range(800..2000) as f64) * 1.5;
        let mut res = format!("{:.2}\n", bal);
        let mut i = 0;
        while i < k {
            let cat = ctgr[rng.gen_range(0..ctgr.len())];
            let p = prices[rng.gen_range(0..prices.len())];
            let ck = chks[rng.gen_range(0..chks.len())];
            //res += &format!("{}, {}, {}\n", cat, p, ck);
            res += &format!("{}, {}, {}\n", ck, cat, p);
            i += 1;
        }
        return res[0..res.len() - 1].to_string();
    }

    fn balance_gz(book: &str) -> String {
        let re1 = oregex::Regex::new(r"[^\n. \dA-Za-z]").unwrap();
        let u = re1.replace_all(book, "");
        let re = oregex::Regex::new(r"\n+").unwrap();
        let mut narr: Vec<&str> = re.split(&u).collect::<Vec<&str>>();
        narr.retain(|x| x.len() != 0);
        let total = match narr[0].parse::<f64>() {
            Ok(total) => total,
            Err(_e) => -1.0,
        };
        let mut current = total;
        let mut res = format!("Original Balance: {:.2}", total);
        let mut i = 1;
        while i < narr.len() {
            let l = narr[i].split(" ").collect::<Vec<&str>>();
            let g = match l[2].parse::<f64>() {
                Ok(g) => g,
                Err(_e) => -1.0,
            };
            current -= g;
            res += &format!("\n{} {} {:.2} Balance {:.2}", l[0], l[1], g, current);
            i += 1;
        }
        res += &format!(
            "\nTotal expense  {:.2}\nAverage expense  {:.2}",
            total - current,
            (total - current) / ((narr.len() - 1) as f64)
        );
        return res;
    }

    #[test]
    fn basic() {
        let b1 = r#"
1000.00!=

125 Market !=:125.45
126 Hardware =34.95
127 Video! 7.45
128 Book :14.32
129 Gasoline ::16.10
"#;
        let b2 = r#"
1233.00
125 Hardware;! 24.8?;
123 Flowers 93.5
127 Meat 120.90
120 Picture 34.00
124 Gasoline 11.00
123 Photos;! 71.4?;
122 Picture 93.5
132 Tyres;! 19.00,?;
129 Stamps 13.6
129 Fruits{} 17.6
129 Market;! 128.00?;
121 Gasoline;! 13.6?;
"#;

        let b1sol="Original Balance: 1000.00\n125 Market 125.45 Balance 874.55\n126 Hardware 34.95 Balance 839.60\n127 Video 7.45 Balance 832.15\n128 Book 14.32 Balance 817.83\n129 Gasoline 16.10 Balance 801.73\nTotal expense  198.27\nAverage expense  39.65";
        let b2sol="Original Balance: 1233.00\n125 Hardware 24.80 Balance 1208.20\n123 Flowers 93.50 Balance 1114.70\n127 Meat 120.90 Balance 993.80\n120 Picture 34.00 Balance 959.80\n124 Gasoline 11.00 Balance 948.80\n123 Photos 71.40 Balance 877.40\n122 Picture 93.50 Balance 783.90\n132 Tyres 19.00 Balance 764.90\n129 Stamps 13.60 Balance 751.30\n129 Fruits 17.60 Balance 733.70\n129 Market 128.00 Balance 605.70\n121 Gasoline 13.60 Balance 592.10\nTotal expense  640.90\nAverage expense  53.41";

        dotest(b1, b1sol);
        dotest(b2, b2sol);
    }

    #[test]
    fn random() {
        let mut rng = rand::thread_rng();
        for _ in 0..5 {
            let s = &compose(rng.gen_range(5..10));
            let sol = balance_gz(s);
            dotest(s, &sol);
        }
    }
}
