use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let file = File::open("src/2025/day01/input.txt")?;
    let reader = BufReader::new(file);

    let mut points_at_zero = 0;
    let mut current = 50;

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let (new_current, zeroes) = times_passes_zero(current, &line);
                points_at_zero += zeroes;
                current = new_current;
            }
            Err(e) => println!("Error reading line: {}", e),
        }
    }

    println!("Password: {}", points_at_zero);
    Ok(())
}

// Part 1
fn new_dial_pointer(current: i32, line: &str) -> (i32, i32) {
    let direction = &line[0..1];
    let number: i32 = line[1..].parse().unwrap();
    match direction {
        "L" => {
          let new_pos = (current + (100 - number)) % 100;
          if new_pos == 0 {
            return (new_pos, 1);
          }
          (new_pos, 0)
        },
        "R" => {
          let new_pos = (current + number) % 100;
          if new_pos == 0 {
            return (new_pos, 1);
          }
          (new_pos, 0)
        },
        _ => (current, 0),
    }
}

// Part 2
fn times_passes_zero(current: i32, line: &str) -> (i32, i32) {
    let direction = &line[0..1];
    let number: i32 = line[1..].parse().unwrap();
    match direction {
        "L" => {
          let zeroes = number / 100;
          let new_pos = (100 + current - (number % 100)) % -100;
          if current - (number % 100) <= 0 {
            return (new_pos, if current != 0 { zeroes + 1 } else { zeroes });
          }
          (new_pos, zeroes)
        },
        "R" => {
          let zeroes = number / 100;
          let new_pos = (current + (number % 100)) % 100;
          if current + (number % 100) >= 100 {
            return (new_pos, if current != 0 { zeroes + 1 } else { zeroes });
          }
          (new_pos, zeroes)
        },
        _ => (current, 0),
    }
}