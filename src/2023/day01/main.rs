use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::collections::HashMap;

#[derive(Debug)]
struct CalibrationEntry {
    digit: i32,
    idx: i32
}

lazy_static::lazy_static! {
    static ref DICT: HashMap<char, Vec<&'static str>> = {
        let mut m = HashMap::new();
        m.insert('o', vec!["one"]);
        m.insert('t', vec!["two", "three"]);
        m.insert('f', vec!["four", "five"]);
        m.insert('s', vec!["six", "seven"]);
        m.insert('e', vec!["eight"]);
        m.insert('n', vec!["nine"]);
        m
    };

    static ref DICT_TO_NUM: HashMap<&'static str, i32> = {
        let mut m = HashMap::new();
        m.insert("one", 1);
        m.insert("two", 2);
        m.insert("three", 3);
        m.insert("four", 4);
        m.insert("five", 5);
        m.insert("six", 6);
        m.insert("seven", 7);
        m.insert("eight", 8);
        m.insert("nine", 9);
        m
    };
}

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let file = File::open("src/2023/day01/input.txt")?;
    let reader = BufReader::new(file);

    let mut total = 0;

    for line in reader.lines() {
        match line {
            Ok(line) => {
                total += calibration_value_2(line)?;
            }
            Err(e) => println!("Error reading line: {}", e),
        }
    }

    println!("Total calibration value: {}", total);
    Ok(())
}

fn calibration_value_2(line: String) -> io::Result<i32> {
    let mut entries: Vec<CalibrationEntry> = Vec::new();
    
    for i in 0..line.len() {
        let c = line.chars().nth(i).ok_or(io::Error::new(io::ErrorKind::Other, "Index out of bounds"))?;

        if DICT.contains_key(&c) {
            let possible_words = DICT.get(&c).unwrap();
            for &word_str in possible_words {
                if line[i..].starts_with(word_str) {
                    let num = *DICT_TO_NUM.get(word_str).unwrap();
                    entries.push(CalibrationEntry {
                        digit: num,
                        idx: i as i32
                    });
                }
            }
        } else if c.is_digit(10) {
            entries.push(CalibrationEntry {
                digit: c.to_digit(10).unwrap() as i32,
                idx: i as i32
            });
        }
    }

    entries.sort_by(|a, b| a.idx.cmp(&b.idx));
    let left = entries.first().ok_or(io::Error::new(io::ErrorKind::Other, "No entries found"))?.digit;
    let right = entries.last().ok_or(io::Error::new(io::ErrorKind::Other, "No entries found"))?.digit;

    Ok(left*10 + right)
}

fn calibration_value(line: String) -> io::Result<i32> {
    let mut left = 0;
    let mut right = 0;
    
    for i in 0..line.len() {
        let c = line.chars().nth(i).ok_or(io::Error::new(io::ErrorKind::Other, "Index out of bounds"))?;
        if c.is_digit(10) {
            left = c.to_digit(10).unwrap() as i32;
            break;
        }
    }

    for i in (0..line.len()).rev() {
        let c = line.chars().nth(i).ok_or(io::Error::new(io::ErrorKind::Other, "Index out of bounds"))?;
        if c.is_digit(10) {
            right = c.to_digit(10).unwrap() as i32;
            break;
        }
    }

    Ok(left*10 + right)
}