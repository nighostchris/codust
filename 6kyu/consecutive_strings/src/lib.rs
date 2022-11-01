// https://www.codewars.com/kata/56a5d994ac971f1ac500003e/train/rust

// First Attempt
// pub fn longest_consec(strarr: Vec<&str>, k: usize) -> String {
//     if strarr.len().eq(&0) || k.gt(&strarr.len()) || k.le(&0) {
//         return String::from("");
//     }
//     let mut consecutive_strings: Vec<String> = (0..strarr.len())
//         .into_iter()
//         .map(|index| {
//             if index <= strarr.len() - k {
//                 return (index..index + k).fold(String::from(""), |mut acc, index| {
//                     acc.push_str(strarr[index]);
//                     acc
//                 });
//             }
//             String::from("")
//         })
//         .collect();

//     consecutive_strings.sort_by(|a, b| b.len().cmp(&a.len()));
//     consecutive_strings[0].clone()
// }

pub fn longest_consec(strarr: Vec<&str>, k: usize) -> String {
    if strarr.len().eq(&0) || k.gt(&strarr.len()) || k.le(&0) {
        return String::from("");
    }
    // The rev() is to make sure the leftmost longest string is always picked up by max_by_key
    strarr
        .windows(k)
        .map(|window| window.join(""))
        .rev()
        .max_by_key(|item| item.len())
        .unwrap()
}

#[cfg(test)]
mod longest_consec_test_suite {
    use super::*;

    fn testing(strarr: Vec<&str>, k: usize, exp: &str) -> () {
        assert_eq!(&longest_consec(strarr, k), exp)
    }

    #[test]
    fn basic() {
        testing(
            vec!["zone", "abigail", "theta", "form", "libe", "zas"],
            2,
            "abigailtheta",
        );
        testing(
            vec![
                "ejjjjmmtthh",
                "zxxuueeg",
                "aanlljrrrxx",
                "dqqqaaabbb",
                "oocccffuucccjjjkkkjyyyeehh",
            ],
            1,
            "oocccffuucccjjjkkkjyyyeehh",
        );
        testing(vec![], 3, "");
        testing(
            vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"],
            3,
            "ixoyx3452zzzzzzzzzzzz",
        );
        testing(vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"], 15, "");
        testing(vec!["it", "wkppv", "ixoyx", "3452", "zzzzzzzzzzzz"], 0, "");
    }

    #[test]
    fn random() {
        testing(
            vec![
                "xxxzzxxtt",
                "iokkii",
                "ywwwiyvv",
                "qqqgkkwwwsssvv",
                "xxiiibb",
                "hhhqqlllwwwybbbcvv",
                "bbbuufhhfffe",
                "zzyyyzz",
                "kkkjjjuucctttff",
            ],
            7,
            "ywwwiyvvqqqgkkwwwsssvvxxiiibbhhhqqlllwwwybbbcvvbbbuufhhfffezzyyyzzkkkjjjuucctttff",
        );
        testing(vec!["cblllkk", "wyyookkuuu", "eeekyyyuuudd", "ooorrwwwyydd", "qyyyiiff", "aaasssgggw", "nmmmszzfyyvvjjj", "hhhfffthha", "gykkkkkkggeee", "gvtttqeeiii"], 9, 
        "wyyookkuuueeekyyyuuuddooorrwwwyyddqyyyiiffaaasssgggwnmmmszzfyyvvjjjhhhfffthhagykkkkkkggeeegvtttqeeiii");
        testing(
            vec![
                "wjjjpum",
                "gggmmemp",
                "uuuhhaaarcccuu",
                "ddorrrvvrrrmmmtttkkk",
                "bbbxxxcccct",
                "uuukkl",
            ],
            6,
            "wjjjpumgggmmempuuuhhaaarcccuuddorrrvvrrrmmmtttkkkbbbxxxcccctuuukkl",
        );
        testing(
            vec![
                "bbxxxyyyeesss",
                "qqzzef",
                "rrjdddwww",
                "ffnnnaaanw",
                "bbppykfll",
                "dddlllbblluuuyy",
                "ejjjffoo",
                "uuuebbgdxxx",
                "rrrppppppijvvvgggww",
                "ppbbbkkkaaaxxxfrrhhhttt",
            ],
            3,
            "uuuebbgdxxxrrrppppppijvvvgggwwppbbbkkkaaaxxxfrrhhhttt",
        );
        testing(
            vec![
                "hhnzzznnnn",
                "dddqqqskkkxxqq",
                "vdpuuooo",
                "dnllljjjtt",
                "mxxxzzzzz",
                "hhhyyygggrxwww",
                "dddttttkk",
                "nccsiiiww",
                "lllffffwwffyyybbbuuu",
            ],
            5,
            "mxxxzzzzzhhhyyygggrxwwwdddttttkknccsiiiwwlllffffwwffyyybbbuuu",
        );
        testing(
            vec![
                "dddryju",
                "xfeeexxxrtttkkkss",
                "llyyyzzppp",
                "iiibee",
                "ccurlucccn",
                "mccbbb",
            ],
            5,
            "dddryjuxfeeexxxrtttkkkssllyyyzzpppiiibeeccurlucccn",
        );
        testing(
            vec![
                "llloosssulx",
                "bkkkiiigaaqq",
                "iifffaaad",
                "wwhaa",
                "izzziifpppk",
                "uuufffllll",
                "rrrskkkl",
                "tmfpmmlll",
                "ffftfffdnnn",
            ],
            8,
            "llloosssulxbkkkiiigaaqqiifffaaadwwhaaizzziifpppkuuufffllllrrrskkkltmfpmmlll",
        );
        testing(
            vec![
                "cceeehhhiii",
                "gddneepbb",
                "nnxxxtmmmii",
                "aaaamvjj",
                "tttwwzoooo",
                "hhwwwjgauyy",
                "eiaooos",
                "cqjttevvvkkkiilll",
                "yzzzzz",
            ],
            5,
            "aaaamvjjtttwwzoooohhwwwjgauyyeiaoooscqjttevvvkkkiilll",
        );
        testing(
            vec![
                "nnbbsssv",
                "wwwaavfffw",
                "mmmqaaappplfffw",
                "qqwffccc",
                "llouuaa",
                "iiiqfnhuujj",
                "bffwwx",
                "yyllwwoo",
                "gghsss",
            ],
            3,
            "nnbbsssvwwwaavfffwmmmqaaappplfffw",
        );
        testing(
            vec![
                "ziiiuz",
                "ffsgggcccee",
                "iixxxrrigggxxx",
                "yyoooa",
                "aaagguhhhcck",
                "fflmmm",
                "iaaxxzziiic",
                "hiibddjkkvee",
            ],
            1,
            "iixxxrrigggxxx",
        );
        testing(
            vec![
                "zzcccmmmgggnm",
                "mmmrrrqqj",
                "zvvv",
                "taaadddzzzkkka",
                "llouuezmm",
                "uuujjzzz",
                "zhhzz",
                "vvvvvtttttvw",
                "hhhuuiieexuu",
            ],
            6,
            "taaadddzzzkkkallouuezmmuuujjzzzzhhzzvvvvvtttttvwhhhuuiieexuu",
        );

        testing(
            vec![
                "wwwjjbbbjjjttt",
                "vdnnn",
                "vvgguuu",
                "vsappp",
                "hhpppffflllhh",
            ],
            2,
            "wwwjjbbbjjjtttvdnnn",
        );
        testing(
            vec!["xxqqkkk", "iiiggvtttx", "wwwppuuu", "byyywyy", "hhzkkpppf"],
            3,
            "xxqqkkkiiiggvtttxwwwppuuu",
        );
        testing(
            vec![
                "ssswwwfmmff",
                "iiiwwwyttt",
                "rrrtttjjjj",
                "fffqtlllo",
                "zzzjjjuzz",
                "qqrrjjjfffppp",
                "ookkknnnhhhtt",
            ],
            5,
            "rrrtttjjjjfffqtlllozzzjjjuzzqqrrjjjfffpppookkknnnhhhtt",
        );
        testing(
            vec![
                "dvqqyyyc",
                "etttnnn",
                "ddrrq",
                "zllkkkyyyb",
                "kkkaaaybb",
                "xxxmmmkvvv",
                "nnmmh",
                "ccmmmzzzw",
            ],
            2,
            "zllkkkyyybkkkaaaybb",
        );
        testing(
            vec![
                "kkkffxccce",
                "qel",
                "sbvcc",
                "ocuusss",
                "zzzooccczzziii",
                "btttdddojj",
                "ixxxvv",
                "ssszzsss",
            ],
            3,
            "ocuussszzzooccczzziiibtttdddojj",
        );
        testing(
            vec![
                "hhmmm",
                "rqxxxq",
                "iiqqqgggh",
                "vvnnndd",
                "qqgxxpj",
                "iuuujmf",
            ],
            5,
            "rqxxxqiiqqqggghvvnnnddqqgxxpjiuuujmf",
        );
        testing(
            vec![
                "cch",
                "hjjrccy",
                "tttuxbbb",
                "nnnllpppooo",
                "mmpkkii",
                "qiiiii",
                "ggxxxpppbz",
            ],
            0,
            "",
        );
        testing(
            vec![
                "nbbjjjaaauuu",
                "yybbbinaaa",
                "pppzzlllg",
                "sssnnnonn",
                "vvvcccyyyxx",
                "cckkkeeqq",
                "oorhhw",
                "ffhb",
                "bbbmmzzz",
            ],
            2,
            "nbbjjjaaauuuyybbbinaaa",
        );
        testing(
            vec![
                "xxxvvvnn",
                "lgrrrcccee",
                "wffuuuqqjj",
                "yypppddddd",
                "xxxfdkkk",
                "zvvvcccggg",
                "bbbeeeffiii",
                "aagggkkn",
                "ddraaa",
            ],
            9,
            "xxxvvvnnlgrrrccceewffuuuqqjjyypppdddddxxxfdkkkzvvvcccgggbbbeeeffiiiaagggkknddraaa",
        );
        testing(
            vec!["ffaiccttt", "smmccc", "rcccuccck", "nooll", "uvvvoooqq"],
            4,
            "ffaicctttsmmcccrcccucccknooll",
        );
        testing(
            vec![
                "ebbbxyyy",
                "qmmmdd",
                "uuooouuwww",
                "nhhhii",
                "slllkkkppp",
                "nuubbbttt",
            ],
            6,
            "ebbbxyyyqmmmdduuooouuwwwnhhhiislllkkkpppnuubbbttt",
        );
        testing(
            vec![
                "jvvmmmiiii",
                "jjjnuu",
                "nhhhq",
                "vmmmvjj",
                "guuuiilss",
                "lbbbaaggg",
                "ueeehdss",
                "fbiiikk",
                "inr",
                "btttww",
            ],
            2,
            "guuuiilsslbbbaaggg",
        );
        testing(
            vec![
                "nooeeezc",
                "reemmmvu",
                "knnymm",
                "ozzzzzz",
                "ccaqqjjjgg",
                "dddfmz",
                "xxxiinnn",
                "nnnjjjjjnnncc",
                "dddzzzxx",
            ],
            6,
            "ozzzzzzccaqqjjjggdddfmzxxxiinnnnnnjjjjjnnnccdddzzzxx",
        );
        testing(
            vec!["ttggyyyk", "bbbeeeqqq", "kkkopp", "lqhhcaa", "llzzzggg"],
            1,
            "bbbeeeqqq",
        );
        testing(
            vec![
                "aaaffflllqq",
                "llkzeee",
                "hhhqquuooxxx",
                "dxxxkbb",
                "ffgiii",
                "zvvvii",
                "ggggsss",
            ],
            4,
            "aaaffflllqqllkzeeehhhqquuooxxxdxxxkbb",
        );
        testing(
            vec![
                "qhddvxxx",
                "iiknnn",
                "hhssns",
                "vvjjggss",
                "ouzzzgg",
                "vvmmo",
                "frrrssse",
                "ojjrrlllkkk",
                "hhoovv",
            ],
            2,
            "frrrssseojjrrlllkkk",
        );
        testing(
            vec![
                "ooossod",
                "qqbbrrg",
                "yyycccddm",
                "gggrddduu",
                "iiiooj",
                "xxxpppeeeee",
                "iinooonnbbb",
                "thhhrnnn",
                "uuuvvvrrx",
            ],
            3,
            "xxxpppeeeeeiinooonnbbbthhhrnnn",
        );
        testing(
            vec![
                "gggeemm",
                "uuuuxtt",
                "ddghhh",
                "kkkvvvgsoo",
                "eln",
                "xxxmcccc",
                "lllzzhhm",
                "zmmmee",
                "lllkkq",
            ],
            3,
            "uuuuxttddghhhkkkvvvgsoo",
        );

        testing(
            vec![
                "xqpo", "zrpc", "snc", "rooi", "zgtox", "uyor", "rcyw", "lhnv",
            ],
            3,
            "rooizgtoxuyor",
        );
        testing(
            vec!["bwt", "prqg", "jnw", "lcjs", "hgsev", "kwux", "mucv"],
            2,
            "lcjshgsev",
        );
        testing(
            vec![
                "illp", "xksl", "rewv", "nak", "uzexw", "ojn", "anfq", "tlyye",
            ],
            8,
            "illpxkslrewvnakuzexwojnanfqtlyye",
        );
        testing(
            vec!["gnind", "tlq", "uwc", "tpd", "eqgf", "fiws"],
            6,
            "gnindtlquwctpdeqgffiws",
        );
        testing(
            vec!["zvw", "uptym", "jci", "mrd", "lazt", "mfs", "sfv"],
            4,
            "uptymjcimrdlazt",
        );
        testing(vec!["xol", "vpq", "shj", "rcji", "ophj"], 3, "shjrcjiophj");
        testing(
            vec!["irlg", "vkd", "tgxa", "uuj", "afh", "intcd"],
            1,
            "intcd",
        );
        testing(
            vec!["bvsa", "dhp", "owq", "loj", "ouwev", "tjwn"],
            1,
            "ouwev",
        );
        testing(
            vec!["vpdx", "cxi", "ieb", "mhmx", "ytdd"],
            5,
            "vpdxcxiiebmhmxytdd",
        );
        testing(
            vec!["rhb", "wfj", "hlc", "yaf", "brhv", "mvv"],
            4,
            "wfjhlcyafbrhv",
        );
        testing(
            vec![
                "loxo", "aey", "iipuj", "evby", "qjlx", "yjkyi", "mkn", "ewor",
            ],
            4,
            "iipujevbyqjlxyjkyi",
        );
        testing(
            vec![
                "dnjz", "qsr", "rnxog", "dlfx", "hqja", "udzi", "iang", "whz",
            ],
            7,
            "dnjzqsrrnxogdlfxhqjaudziiang",
        );
        testing(
            vec![
                "jdkze", "wxd", "zmvi", "aapwl", "dmjz", "eqxuh", "yeppe", "wpwu",
            ],
            2,
            "eqxuhyeppe",
        );
        testing(
            vec!["jfby", "fsoc", "vfil", "ffyw", "npbix"],
            5,
            "jfbyfsocvfilffywnpbix",
        );
        testing(
            vec![
                "ampy", "tycy", "mukzo", "giky", "qyr", "cny", "hvwh", "acogo",
            ],
            1,
            "mukzo",
        );
        testing(
            vec!["ibu", "xlho", "bgzvd", "tbs", "bjqk", "jdhx"],
            1,
            "bgzvd",
        );
    }
}
