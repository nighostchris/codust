// https://www.codewars.com/kata/54dc6f5a224c26032800005c/train/rust

use std::collections::HashMap;

// First Attempt
// pub fn stock_list(list_art: Vec<&str>, list_cat: Vec<&str>) -> String {
//     if list_art.len().eq(&0) || list_cat.len().eq(&0) {
//         return String::from("");
//     }

//     let mut stock_hash_map = HashMap::new();

//     for &category in list_cat.iter() {
//         stock_hash_map.insert(category.to_string(), 0);
//     }

//     for stock in list_art {
//         let split: Vec<&str> = stock.split(" ").collect();
//         let key = split.get(0).unwrap().get(0..1).unwrap().to_string();
//         let value = split.get(1).unwrap().parse::<i32>().unwrap();

//         stock_hash_map.entry(key).and_modify(|count| *count += value);
//     }

//     let output = list_cat
//         .iter()
//         .map(|&category| format!("({} : {})", category, stock_hash_map.get(category).unwrap()))
//         .collect::<Vec<String>>()
//         .join(" - ");
//     output.to_string()
// }

pub fn stock_list(list_art: Vec<&str>, list_cat: Vec<&str>) -> String {
    if list_art.len().eq(&0) || list_cat.len().eq(&0) {
        return String::from("");
    }

    let mut stock_hash_map = HashMap::new();

    for stock in list_art {
        let split: Vec<&str> = stock.split(" ").collect();
        let value = split.get(1).unwrap().parse::<i32>().unwrap();

        *stock_hash_map.entry(&stock[0..1]).or_insert(0) += value;
    }

    list_cat
        .iter()
        .map(|&category| {
            format!(
                "({} : {})",
                category,
                stock_hash_map.get(category).unwrap_or(&0)
            )
        })
        .collect::<Vec<String>>()
        .join(" - ")
        .to_string()
}

#[cfg(test)]
mod stock_list_test_suite {
    use super::*;

    fn dotest(list_art: Vec<&str>, list_cat: Vec<&str>, exp: &str) -> () {
        println!("list_art: {:?};", list_art);
        println!("list_cat: {:?};", list_cat);
        let ans = stock_list(list_art, list_cat);
        println!("actual:\n{:?};", ans);
        println!("expect:\n{:?};", exp);
        println!("{};", ans == exp);
        assert_eq!(ans, exp);
        println!("{};", "-");
    }

    #[test]
    fn basic() {
        let mut b = vec!["BBAR 150", "CDXE 515", "BKWR 250", "BTSQ 890", "DRTY 600"];
        let mut c = vec!["A", "B", "C", "D"];
        dotest(b, c, "(A : 0) - (B : 1290) - (C : 515) - (D : 600)");

        b = vec!["ABAR 200", "CDXE 500", "BKWR 250", "BTSQ 890", "DRTY 600"];
        c = vec!["A", "B"];
        dotest(b, c, "(A : 200) - (B : 1140)");

        b = vec!["CBART 20", "CDXEF 50", "BKWRK 25", "BTSQZ 89", "DRTYM 60"];
        c = vec!["A", "B", "C", "W"];
        dotest(b, c, "(A : 0) - (B : 114) - (C : 70) - (W : 0)");

        b = vec![
            "ROXANNE 102",
            "RHODODE 123",
            "BKWRKAA 125",
            "BTSQZFG 239",
            "DRTYMKH 060",
        ];
        c = vec!["B", "R", "D", "X"];
        dotest(b, c, "(B : 364) - (R : 225) - (D : 60) - (X : 0)");

        b = vec![];
        c = vec!["B", "R", "D", "X"];
        dotest(b, c, "");

        b = vec![
            "ROXANNE 102",
            "RHODODE 123",
            "BKWRKAA 125",
            "BTSQZFG 239",
            "DRTYMKH 060",
        ];
        c = vec![];
        dotest(b, c, "");

        b = vec![
            "ROXANNE 102",
            "RHODODE 123",
            "BKWRKAA 125",
            "BTSQZFG 239",
            "DRTYMKH 060",
        ];
        c = vec!["U", "V", "R"];
        dotest(b, c, "(U : 0) - (V : 0) - (R : 225)");
    }

    fn stock_list_ba(list_art: Vec<&str>, list_cat: Vec<&str>) -> String {
        if list_art.len() == 0 || list_cat.len() == 0 {
            return "".to_string();
        }
        let mut res = "".to_string();
        for cat in list_cat {
            let mut total = 0;
            for i in 0..list_art.len() {
                let book = list_art[i];
                if &book[0..1] == &cat[0..1] {
                    let u = book.split(" ").collect::<Vec<&str>>()[1];
                    total += u.parse::<i32>().unwrap();
                }
            }
            if res.len() != 0 {
                res += &format!(" {} ", "-");
            }
            res += &format!("({} : {})", cat, total);
        }
        return res;
    }

    extern crate rand;
    use self::rand::seq::SliceRandom;
    use self::rand::{thread_rng, Rng};

    fn compose() -> (Vec<&'static str>, Vec<&'static str>) {
        let mut d = vec![
            "BBAR 150",
            "CDXE 515",
            "BKWR 250",
            "BTSQ 890",
            "DRTY 600",
            "ABAR 200",
            "CDXEF 500",
            "BKWRW 250",
            "BTSQA 890",
            "DRTYU 600",
            "CBART 20",
            "CDXEG 50",
            "BKWRK 25",
            "BTSQZ 89",
            "DRTYM 60",
            "ROXANNE 102",
            "RHODODE 123",
            "BKWRKAA 125",
            "BTSQZFG 239",
            "DRTYMKH 060",
            "ROXANNEB 102",
            "RHODODEA 123",
            "BKWRKAB 125",
            "BTSQZA 239",
            "DRTYMKA 060",
            "ROXANNEA 102",
            "RHODODEB 123",
            "BKWRKAC 125",
            "BTSQZB 239",
            "DRTYMKB 060",
            "ROXANNEC 102",
            "RHODODEC 123",
            "BKWRKAD 125",
            "BTSQZC 239",
            "DRTYMC 060",
            "RHIB 1230",
            "RO 530",
            "XILO 32",
            "ROXANNEZ 102",
            "RHODODED 123",
            "BKWRKAE 125",
            "BTSQZD 239",
            "DRTYMD 060",
            "RHIBA 1230",
            "ROA 530",
            "XILOA 32",
            "UZO 32000",
            "ROXANNES 102",
            "RHODODEF 123",
            "BKWRKAF 125",
            "BTSQZE 239",
            "DRTYME 060",
            "RHIBB 1230",
            "ROB 530",
            "XILOB 32",
        ];
        let mut l = vec!["A", "B", "C", "F", "D", "R", "U", "X", "W"];
        let mut rng = rand::thread_rng();
        d.shuffle(&mut thread_rng());
        l.shuffle(&mut thread_rng());
        let r = rng.gen_range(5..10);
        let la = &d[1..r];
        let rr = rng.gen_range(4..6);
        let lc = &l[1..rr];
        return (la.to_vec(), lc.to_vec());
    }

    #[test]
    fn random() {
        for _ in 0..100 {
            let (la, lc) = compose();
            let laa = la.clone();
            let lcc = lc.clone();
            let sol = &stock_list_ba(la, lc);
            dotest(laa, lcc, sol);
        }
    }
}
