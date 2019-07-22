#[macro_use]
extern crate lazy_static;
extern crate regex;

use regex::Regex;

pub fn translate(input: &str) -> String {
    input
        .split(" ")
        .map(|s| pig_latin(s))
        .collect::<Vec<_>>()
        .join(" ")
}

fn pig_latin(s: &str) -> String {
    lazy_static! {
        static ref VOWEL: Regex = Regex::new(r"^(?:(?:[aeiou])|(?:xr)|(?:yt)).*$").unwrap();
        static ref CONSONANT: Regex =
            Regex::new(r"(?:(?P<a>[^aeiou]?qu)|(?P<b>[^aeiou][^aeiouy]*))(?P<x>.*)$").unwrap();
    }
    if VOWEL.is_match(s) {
        s.to_string() + "ay"
    } else {
        CONSONANT.replace(s, "$x$a$b").to_string() + "ay"
    }
}
