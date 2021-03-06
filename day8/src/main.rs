use std::collections::BTreeSet;
use std::fmt;
use std::io::{BufRead, BufReader};
use std::num::ParseIntError;
use std::str::FromStr;

// cat <<EOF | cargo run
// be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
// edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
// fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
// fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
// aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
// fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
// dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
// bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
// egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
// gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
// EOF

#[derive(Debug, Clone)]
struct ParseError;

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not parse")
    }
}

#[derive(Debug)]
struct SevenSegDisp {
    top: Option<char>,
    upper_left: Option<char>,
    upper_right: Option<char>,
    middle: Option<char>,
    lower_left: Option<char>,
    lower_right: Option<char>,
    bottom: Option<char>,
}

//      0:      1:      2:      3:      4:
//     aaaa    ....    aaaa    aaaa    ....
//    b    c  .    c  .    c  .    c  b    c
//    b    c  .    c  .    c  .    c  b    c
//     ....    ....    dddd    dddd    dddd
//    e    f  .    f  e    .  .    f  .    f
//    e    f  .    f  e    .  .    f  .    f
//     gggg    ....    gggg    gggg    ....
//
//      5:      6:      7:      8:      9:
//     aaaa    aaaa    aaaa    aaaa    aaaa
//    b    .  b    .  .    c  b    c  b    c
//    b    .  b    .  .    c  b    c  b    c
//     dddd    dddd    ....    dddd    dddd
//    .    f  e    f  .    f  e    f  .    f
//    .    f  e    f  .    f  e    f  .    f
//     gggg    gggg    ....    gggg    gggg

impl FromStr for SevenSegDisp {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let segs = s
            .to_string()
            .split_whitespace()
            .map(|n| n.parse::<String>().ok())
            .collect::<Vec<_>>();

        let mut one: Option<String> = None;
        let mut four: Option<String> = None;
        let mut seven: Option<String> = None;
        let mut sixninezero: Vec<String> = Vec::new();

        for segment in segs.clone() {
            let segment = segment.unwrap();
            match segment.len() {
                2 => one = Some(segment.clone()),
                4 => four = Some(segment.clone()),
                3 => seven = Some(segment.clone()),
                6 => sixninezero.push(segment.clone()),
                _ => (),
            }
        }
        if one.is_none() || four.is_none() || seven.is_none() {
            unreachable!();
        }
        let one = one.unwrap();
        let four = four.unwrap();
        let seven = seven.unwrap();
        if sixninezero.len() != 3 {
            unreachable!();
        }
        Ok(SevenSegDisp::new(one, four, seven, sixninezero).unwrap())
    }
}

impl SevenSegDisp {
    fn parse(&self, val: String) -> Result<usize, ParseError> {
        let mut tmp = SevenSegDisp {
            top: None,
            upper_left: None,
            upper_right: None,
            middle: None,
            lower_left: None,
            lower_right: None,
            bottom: None,
        };
        for c in val.chars().collect::<Vec<char>>() {
            if c == self.top.unwrap() {
                tmp.top = Some('_');
            } else if c == self.upper_left.unwrap() {
                tmp.upper_left = Some('_');
            } else if c == self.upper_right.unwrap() {
                tmp.upper_right = Some('_');
            } else if c == self.middle.unwrap() {
                tmp.middle = Some('_');
            } else if c == self.lower_left.unwrap() {
                tmp.lower_left = Some('_');
            } else if c == self.lower_right.unwrap() {
                tmp.lower_right = Some('_');
            } else if c == self.bottom.unwrap() {
                tmp.bottom = Some('_');
            } else {
                unreachable!();
            }
        }
        match (
            tmp.top,
            tmp.upper_left,
            tmp.upper_right,
            tmp.middle,
            tmp.lower_left,
            tmp.lower_right,
            tmp.bottom,
        ) {
            (Some(_), Some(_), Some(_), Some(_), Some(_), Some(_), Some(_)) => Ok(8),
            (Some(_), Some(_), Some(_), None, Some(_), Some(_), Some(_)) => Ok(0),
            (Some(_), Some(_), Some(_), Some(_), None, Some(_), Some(_)) => Ok(9),
            (Some(_), Some(_), None, Some(_), Some(_), Some(_), Some(_)) => Ok(6),
            (Some(_), None, Some(_), Some(_), Some(_), None, Some(_)) => Ok(2),
            (Some(_), None, Some(_), Some(_), None, Some(_), Some(_)) => Ok(3),
            (Some(_), Some(_), None, Some(_), None, Some(_), Some(_)) => Ok(5),
            (None, Some(_), Some(_), Some(_), None, Some(_), None) => Ok(4),
            (Some(_), None, Some(_), None, None, Some(_), None) => Ok(7),
            (None, None, Some(_), None, None, Some(_), None) => Ok(1),
            _ => unreachable!(),
        }
    }

    fn new(
        one: String,
        four: String,
        seven: String,
        sixninezero: Vec<String>,
    ) -> Result<SevenSegDisp, ParseError> {
        let eight: BTreeSet<char> = BTreeSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g']);
        let one: BTreeSet<char> = one.chars().collect();
        let four: BTreeSet<char> = four.chars().collect();
        let seven: BTreeSet<char> = seven.chars().collect();
        let sixninezero: BTreeSet<BTreeSet<char>> =
            sixninezero.iter().fold(BTreeSet::new(), |mut acc, x| {
                acc.insert(x.chars().collect());
                acc
            });
        let nine = sixninezero.iter().find(|x| four.is_subset(x)).unwrap();
        let zero = sixninezero
            .iter()
            .find(|x| seven.is_subset(x) && !nine.is_subset(x))
            .unwrap();
        let six = sixninezero
            .iter()
            .find(|x| !x.is_subset(nine) && !x.is_subset(zero))
            .unwrap();
        let top = *seven
            .difference(&one)
            .cloned()
            .collect::<Vec<char>>()
            .get(0)
            .unwrap();
        let lower_left = *eight
            .difference(nine)
            .cloned()
            .collect::<Vec<char>>()
            .get(0)
            .unwrap();
        let middle = *eight
            .difference(zero)
            .cloned()
            .collect::<Vec<char>>()
            .get(0)
            .unwrap();
        let upper_right = *eight
            .difference(six)
            .cloned()
            .collect::<Vec<char>>()
            .get(0)
            .unwrap();
        let lower_right = *one
            .difference(&BTreeSet::from([upper_right]))
            .cloned()
            .collect::<Vec<char>>()
            .get(0)
            .unwrap();
        let bottom: BTreeSet<char> = nine.difference(&four).cloned().collect();
        let bottom = *bottom
            .difference(&BTreeSet::from([top]))
            .cloned()
            .collect::<Vec<char>>()
            .get(0)
            .unwrap();
        let upper_left = *eight
            .difference(&BTreeSet::from([
                top,
                upper_right,
                middle,
                lower_left,
                lower_right,
                bottom,
            ]))
            .cloned()
            .collect::<Vec<char>>()
            .get(0)
            .unwrap();
        Ok(SevenSegDisp {
            top: Some(top),
            upper_left: Some(upper_left),
            upper_right: Some(upper_right),
            middle: Some(middle),
            lower_left: Some(lower_left),
            lower_right: Some(lower_right),
            bottom: Some(bottom),
        })
    }
}

fn main() {
    // Read lines like:
    //   be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
    let sum: usize = BufReader::new(std::io::stdin())
        .lines()
        .map(|line| {
            // Split each line on '|' character, passing the two pieces into the match
            match line.unwrap().split(" | ").collect::<Vec<&str>>()[..] {
                [scrambled_digits, to_parse] => {
                    // Process the 10 scrambled digits to decode the wiring
                    let segment =
                        SevenSegDisp::from_str(scrambled_digits).expect("Failed to parse!");
                    // Run the 4 digits through the display to decode them
                    to_parse
                        .to_string()
                        .split_whitespace()
                        .map(|n| {
                            segment
                                .parse(n.to_string())
                                .expect("failed to parse")
                                .to_string()
                        })
                        .collect::<Vec<_>>()
                        // Return the string-concatenated representation of the 4 digits, i.e.
                        // "1234"
                        .join("")
                }
                _ => unreachable!(),
            }
            // Parse the string-representation of the digits as a number, i.e. "1234" -> 1234
            .parse::<usize>()
            .unwrap()
        })
        // Sum all the numbers
        .sum();
    println!("{:?} == 996280", sum);
}
