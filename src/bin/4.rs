use std::fs::File;
use std::io::prelude::*;

fn main() {
    assert!(no_duplicate_words("aa bb cc dd ee"));
    assert!(!no_duplicate_words("aa bb cc dd aa"));
    assert!(no_duplicate_words("aa bb cc dd aaa"));

    assert!(no_anagram_words("abcde fghij"));
    assert!(!no_anagram_words("abcde xyz ecdab"));
    assert!(no_anagram_words("a ab abc abd abf abj"));
    assert!(no_anagram_words("iiii oiii ooii oooi oooo"));
    assert!(!no_anagram_words("oiii ioii iioi iiio"));

    let mut f = File::open("4.in").expect("file not found");
    let mut contents = String::new();
    f.read_to_string(&mut contents).expect("blug");

    let result = contents
        .split("\n")
        .filter(|s| s.split_whitespace().count() > 1) // remove blanks etc
        .filter(|s| no_duplicate_words(&s))
        // .inspect(|s| println!("{:?}", s))
        .count();

    println!("part 1: {}", result);


    let result2 = contents
        .split("\n")
        .filter(|s| s.split_whitespace().count() > 1) // remove blanks etc
        .filter(|s| no_anagram_words(&s))
        .inspect(|s| println!("{:?}", s))
        .count();
    println!("part 2: {}", result2);
}

fn no_duplicate_words(phrase: &str) -> bool {
    let word_count = phrase.split_whitespace().count();

    let mut words = phrase.split_whitespace().collect::<Vec<&str>>();
    words.sort();
    words.dedup();
    return word_count == words.len();
}

fn no_anagram_words(phrase: &str) -> bool {
    let word_count = phrase.split_whitespace().count();

    let mut deduped = phrase.split_whitespace().map(sort_string).collect::<Vec<String>>();
    deduped.sort();
    deduped.dedup();
    return word_count == deduped.len();
}

fn sort_string(word: &str) -> String {
    let mut v: Vec<char> = word.chars().collect::<Vec<char>>();
    v.sort();
    return v.iter().collect();
}
