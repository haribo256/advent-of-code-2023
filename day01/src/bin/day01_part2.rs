use anyhow::Result;
use std::{
    collections::HashMap,
    io::{BufRead, Cursor},
};

fn main() -> Result<()> {
    let mut sum = 0i32;

    let word_values = [
        (String::from("zero"), 0),
        (String::from("one"), 1),
        (String::from("two"), 2),
        (String::from("three"), 3),
        (String::from("four"), 4),
        (String::from("five"), 5),
        (String::from("six"), 6),
        (String::from("seven"), 7),
        (String::from("eight"), 8),
        (String::from("nine"), 9),
    ];

    let number_words = (0..=9).into_iter().map(|x| (x.to_string(), x));
    let string_values = word_values.into_iter().chain(number_words);
    let word_or_numeric_values: HashMap<String, i32> = HashMap::from_iter(string_values);

    for word_or_numeric_value in word_or_numeric_values.iter() {
        println!(
            "Search Value: {}={}",
            word_or_numeric_value.0, word_or_numeric_value.1
        );
    }

    let input = include_str!("input.txt");
    let cursor = Cursor::new(input);

    for line in cursor.lines() {
        let line = match line {
            Ok(line) => line,
            Err(e) => panic!("Error reading line: {:?}", e),
        };

        print!("* {}: ", line);

        let search_iter = word_or_numeric_values
            .iter()
            .flat_map(|x| line.match_indices(x.0).map(move |y| (x.0, x.1, y.0)));

        let Some((_, first_numeric_value, _)) =
            search_iter.clone().min_by_key(|word_entry| word_entry.2)
        else {
            panic!("Could not find first number")
        };

        let Some((_, last_numeric_value, _)) =
            search_iter.clone().max_by_key(|word_entry| word_entry.2)
        else {
            panic!("Could not find last number")
        };

        let joined_numeric_string = format!("{}{}", first_numeric_value, last_numeric_value);
        let joined_numeric_value = u8::from_str_radix(&joined_numeric_string, 10)?;

        if joined_numeric_value == 0 {
            panic!("We should not have any zeros");
        }

        let previous_sum = sum;

        sum += joined_numeric_value as i32;

        println!("{} + {} = {}", previous_sum, joined_numeric_value, sum);
    }

    println!("total: {}", sum);

    Ok(())
}
