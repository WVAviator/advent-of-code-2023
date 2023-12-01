use crate::utils::to_lines_vec;

use super::Challenge;
use std::fs::File;

#[derive(Default)]
pub struct Trebuchet {
    lines: Vec<String>,
}

impl Challenge for Trebuchet {
    fn load(&mut self, file: &File) {
        self.lines = to_lines_vec(file);
    }
    fn solve_part_one(&self) -> String {
        let mut values: Vec<u32> = Vec::new();
        for line in self.lines.clone() {
            let (first, last) = Trebuchet::get_first_and_last_digits(&line);
            let num_str = format!("{}{}", first, last);
            values.push(num_str.parse::<u32>().unwrap());
        }

        let sum = values.iter().sum::<u32>();

        format!("{}", sum)
    }
    fn solve_part_two(&self) -> String {
        let mut values = Vec::new();
        for line in self.lines.clone() {
            let (first, last) = Trebuchet::get_first_and_last_numstr_or_digit(&line);
            let num_str = format!("{}{}", first, last);
            println!("{}: {}", line, num_str);
            values.push(num_str.parse::<u32>().unwrap());
        }

        let sum = values.iter().sum::<u32>();

        format!("{}", sum)
    }
}

impl Trebuchet {
    fn get_first_and_last_digits(line: &str) -> (char, char) {
        let digits: Vec<char> = line.chars().filter(|c| c.is_digit(10)).collect();
        if digits.len() == 0 {
            panic!("No digits found in line: {}", line);
        }
        (digits[0], digits[digits.len() - 1])
    }

    fn find_all_number_substrings(input: &str) -> Vec<char> {
        let substrings = [
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "1", "2", "3",
            "4", "5", "6", "7", "8", "9",
        ];
        let mut found_substrings = Vec::new();

        for (i, &substring) in substrings.iter().enumerate() {
            input.match_indices(substring).for_each(|(str_index, _)| {
                found_substrings.push((str_index, i));
            });
        }

        found_substrings.sort_by_key(|&(str_index, _)| str_index);
        found_substrings
            .iter()
            .map(|&(_, substrings_index)| {
                char::from_digit(((substrings_index % 9) + 1) as u32, 10).unwrap()
            })
            .collect()
    }

    fn get_first_and_last_numstr_or_digit(line: &str) -> (char, char) {
        let substrings = Trebuchet::find_all_number_substrings(line);
        if substrings.len() == 0 {
            panic!("No digits found in line: {}", line);
        }
        let first = substrings[0];
        let last = substrings[substrings.len() - 1];

        (first, last)
    }
}

mod test {
    use super::*;

    #[test]
    fn get_first_and_last_digit() {
        let line = "aerv5ndsfjvn8anrve9erferf2fwef";
        let (first, last) = Trebuchet::get_first_and_last_digits(line);
        assert_eq!(first, '5');
        assert_eq!(last, '2');
    }

    #[test]
    fn get_first_and_last_digit_with_one_digit() {
        let line = "esirvioernivoen5aiorjnvioerjogeij";
        let (first, last) = Trebuchet::get_first_and_last_digits(line);
        assert_eq!(first, '5');
        assert_eq!(last, '5');
    }

    #[test]
    fn test_part_one() {
        let mut trebuchet = Trebuchet::default();
        trebuchet.lines = vec![
            String::from("1abc2"),
            String::from("pqr3stu8vwx"),
            String::from("a1b2c3d4e5f"),
            String::from("treb7uchet"),
        ];
        assert_eq!(trebuchet.solve_part_one(), "142");
    }

    #[test]
    fn get_first_and_last_numstr_or_digit() {
        let line = "rehfujoneruvjeu4rjvijefiveuerfijer8fnjrnbv";
        let (first, last) = Trebuchet::get_first_and_last_numstr_or_digit(line);
        assert_eq!(first, '1');
        assert_eq!(last, '8');
    }

    #[test]
    fn get_first_and_last_numstr_or_digit_with_one_digit() {
        let line = "eruvjneiruvnfoureruvn";
        let (first, last) = Trebuchet::get_first_and_last_numstr_or_digit(line);
        assert_eq!(first, '4');
        assert_eq!(last, '4');
    }

    #[test]
    fn find_all_number_substrings() {
        let line = "seightwooneqxcfgszninesvfcnxc68";
        let substrings = Trebuchet::find_all_number_substrings(line);
        assert_eq!(substrings, vec!['8', '2', '1', '9', '6', '8']);
    }

    #[test]
    fn find_all_number_substrings_single() {
        let line = "eruvjneiruvnfoureruvn";
        let substrings = Trebuchet::find_all_number_substrings(line);
        assert_eq!(substrings, vec!['4']);
    }

    #[test]
    fn find_all_number_substrings_none() {
        let line = "eruvjneiruvnforeruvn";
        let substrings = Trebuchet::find_all_number_substrings(line);
        assert!(substrings.is_empty());
    }

    #[test]
    fn final_all_number_substrings_repeated_substrings() {
        let line = "oneone33oneonefourfourfouroneonefour33four";
        let substrings = Trebuchet::find_all_number_substrings(line);
        assert_eq!(
            substrings,
            vec!['1', '1', '3', '3', '1', '1', '4', '4', '4', '1', '1', '4', '3', '3', '4']
        );
    }

    #[test]
    fn find_all_number_substrings_finds_all_types() {
        let line = "onetwothreefourfivesixseveneightnine123456789";
        let substrings = Trebuchet::find_all_number_substrings(line);
        assert_eq!(
            substrings,
            vec![
                '1', '2', '3', '4', '5', '6', '7', '8', '9', '1', '2', '3', '4', '5', '6', '7',
                '8', '9'
            ]
        );
    }

    #[test]
    fn test_part_two() {
        let mut trebuchet = Trebuchet::default();
        trebuchet.lines = vec![
            String::from("two1nine"),
            String::from("eightwothree"),
            String::from("abcone2threexyz"),
            String::from("xtwone3four"),
            String::from("4nineeightseven2"),
            String::from("zoneight234"),
            String::from("7pqrstsixteen"),
        ];
        assert_eq!(trebuchet.solve_part_two(), "281");
    }

    #[test]
    fn test_part_two_all_possible() {
        let mut trebuchet = Trebuchet::default();
        trebuchet.lines = vec![
            String::from("one384h3two1"),
            String::from("2hhhhhhhhtwo"),
            String::from("threebvghbvhgb3"),
            String::from("4nevjrkvnerkjtfour"),
            String::from("584952849582five"),
            String::from("sixsevenonetwofour23423nine6"),
            String::from("7seven"),
            String::from("eight"),
            String::from("nine9nine"),
        ];
        assert_eq!(trebuchet.solve_part_two(), "495");
    }

    #[test]
    fn test_part_two_2() {
        let mut trebuchet = Trebuchet::default();
        trebuchet.lines = vec![
            String::from("f3"),                                                // 33
            String::from("6zzrfxdxseventhree"),                                // 63
            String::from("265one"),                                            // 21
            String::from("seven3lbcvjxqhhdpzkttqsixjzzjjbclfq1fiveeightwojx"), // 72
            String::from("seightwoone8qxcfgszninesvfcnxc68"),                  // 88
            String::from("strqnb5eightbpnkcjdz6"),                             // 56
            String::from("fiveninebtpbpjqbgx2bmjrgmprnd"),                     // 52
            String::from("sixgtxr2fourrdkjg"),                                 // 64
            String::from("fivebxsevensixone872dlx"),                           // 52
        ];
        assert_eq!(trebuchet.solve_part_two(), "501");
    }
}
