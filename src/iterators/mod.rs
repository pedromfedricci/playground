#[cfg(test)]
mod test;

mod double_ended;

use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, num::ParseIntError};

pub fn drain_example() {
    let mut v = vec![1, 2, 3, 4, 5];
    {
        // drain will remove the values on drop,
        //
        let iter = v.drain(2..);
        println!("Will be removed from vector: {:?}", iter);
    }
    println!("Drained vector: {:?}", v);
}

pub fn flat_map_example() {
    let mut major_cities = HashMap::new();
    major_cities.insert("Japan", vec!["Tokyo", "Kyoto"]);
    major_cities.insert("Brazil", vec!["Sao Paulo", "Rio de Janeiro"]);
    major_cities.insert("Kenya", vec!["Nairobi", "Mombasa"]);

    let countries = major_cities.keys();
    let flap_map = countries.flat_map(|&country| &major_cities[country]);
    for &city in flap_map {
        println!("{}", city);
    }
}

pub fn scan_example() {
    let a = [1, 2, 3, 4, 5];
    let iter = a.iter().scan(1, |state, &x| {
        *state *= x;
        Some(-*state)
    });
    for item in iter {
        println!("{}", item);
    }
}

pub fn take_while_example() {
    let message = "\
        To: jimb\r\n\
        From: superego <editor@oreilly.com>\r\n\
        \r\n\
        Did you get any writing done today?\r\n\
        When will you stop wasting time plotting fractals?\r\n";
    println!("{}", message);
    println!("{:?}", message.lines().collect::<Vec<_>>());
    // Will stop at line number 3, as it is empty, \r\n will be discarted
    // since that line ends with a leading backslash, resulting in "".
    // Once the condition !line.is_empty() is no longer true, the iterator
    // will not yield any more new elements.
    for header in message.lines().take_while(|&line| !line.is_empty()) {
        println!("{}", header);
    }
}

pub fn skip_while_example() {
    let message = "\
        To: jimb\r\n\
        From: superego <editor@oreilly.com>\r\n\
        \r\n\
        Did you get any writing done today?\r\n\
        When will you stop wasting time plotting fractals?\r\n";
    println!("{}", message);
    println!("{:?}", message.lines().collect::<Vec<_>>());

    message
        .lines()
        .skip_while(|&line| !line.is_empty())
        // The first empty line will return true, the value will be yielded.
        // You can skip a determined number of line with skip().
        .skip(1)
        .for_each(|line| println!("{}", line));
}

pub fn fold_example() {
    let strings = [
        String::from("aaaa"),
        String::from("bbbb"),
        String::from("cccc"),
    ];
    let concat = strings.iter().fold(String::from(""), |mut acc, s| {
        acc.push_str(s);
        acc
    });
    println!("Concatenated string: {}", concat);
}

pub fn parser_example_1() {
    fn parse_next_integer<I>(tokens: &mut I) -> Option<u32>
    where
        I: Iterator<Item = char>,
    {
        let mut acc = 0;
        tokens
            .skip_while(|token| !token.is_digit(10))
            .map_while(|token| token.to_digit(10)) // only on nightly
            .for_each(|digit| acc = acc * 10 + digit);

        if acc > 0 {
            Some(acc)
        } else {
            None
        }
    }

    let string = "12354534,sadsd32321343,weqwew2q2312323e3a32aw3e233e3e23e32e987";
    let mut chars = string.chars();
    let mut vec = Vec::new();

    while let Some(number) = parse_next_integer(&mut chars) {
        vec.push(number);
    }
    println!("{:?}", vec);
    println!("{:?}", string);
}

pub fn parser_example_2() {
    fn parse_next_integer<I>(tokens: &mut I) -> Option<u32>
    where
        I: Iterator<Item = char>,
    {
        let is_digit = |token: &char| token.is_digit(10);
        let not_digit = |token: &char| !is_digit(token);

        tokens
            .skip_while(not_digit)
            .take_while(is_digit)
            .collect::<String>()
            .parse::<u32>()
            .ok()
    }

    let string = u32::MAX.to_string() + "asdsadsadsads" + "1233";
    let mut chars = string.chars();
    let mut vec = Vec::new();

    while let Some(number) = parse_next_integer(&mut chars) {
        vec.push(number);
    }
    println!("{:?}", vec);
    println!("{:?}", string);
}

lazy_static! {
    // Note that if a invalid expression is provided as input,
    // this will cause the caller to panic during runtime.
    static ref RE: Regex = Regex::new(r"-{0,1}\d+").unwrap();
}

pub fn parser_example_3() {
    trait ParserMatches = Iterator<Item = Result<i32, ParseIntError>>;
    fn parser_matches(tokens: &str) -> impl ParserMatches + '_ {
        RE.find_iter(tokens).map(|mat| mat.as_str().parse::<i32>())
    }

    let tokens = "---12354534,sadsd32321343,weqwew-2q2312323e3a32aw3e233e3e23e32e-987";
    let mut matches = parser_matches(tokens);
    while let Some(Ok(integer)) = matches.next() {
        println!("{}", integer);
    }
}

pub fn max_by_example() {
    let mut populations = HashMap::new();
    populations.insert("Portland", 583_778);
    populations.insert("Fosil", 449);
    populations.insert("Grenhorn", 20);
    populations.insert("Boring", 7_787);
    populations.insert("The Dalles", 15_324);

    let max_pop = populations
        .iter()
        .max_by(|left, right| left.1.partial_cmp(right.1).unwrap());
    assert_eq!(max_pop, Some((&"Portland", &583_778)));
}

pub fn max_by_key_example() {
    let mut populations = HashMap::new();
    populations.insert("Portland", 583_778);
    populations.insert("Fosil", 449);
    populations.insert("Grenhorn", 20);
    populations.insert("Boring", 7_787);
    populations.insert("The Dalles", 15_324);

    let max_pop = populations.iter().max_by_key(|&(_name, pop)| pop);
    assert_eq!(max_pop, Some((&"Portland", &583_778)));
}
