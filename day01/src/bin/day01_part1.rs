use anyhow::Result;
use std::io::{BufRead, Cursor};

fn main() -> Result<()> {
    let mut sum = 0i32;

    let input = include_str!("input.txt");
    let cursor = Cursor::new(input);

    for line in cursor.lines() {
        let line = match line {
            Ok(line) => line,
            Err(e) => panic!("Error reading line: {:?}", e),
        };

        print!("* {}: ", line);

        let numeric_iter = line
            .chars()
            .filter(|c| c.is_numeric())
            .map(|c| u8::from_str_radix(c.to_string().as_str(), 10));

        let first_numeric_char = numeric_iter.clone().next();
        let first_numeric_value = match first_numeric_char {
            None => panic!("Could not find first numeric char"),
            Some(Err(e)) => panic!("Error parsing first numeric char {:?}", e),
            Some(Ok(v)) => v,
        };

        let last_numeric_char = numeric_iter.clone().last();
        let last_numeric_value = match last_numeric_char {
            None => panic!("Could not find last numeric char"),
            Some(Err(e)) => panic!("Error parsing last numeric char {:?}", e),
            Some(Ok(v)) => v,
        };

        let joined_numeric_string = format!("{}{}", first_numeric_value, last_numeric_value);
        let joined_numeric_value = u8::from_str_radix(&joined_numeric_string, 10)?;

        sum += joined_numeric_value as i32;

        println!("{}", joined_numeric_value);
    }

    println!("total: {}", sum);

    Ok(())
}
