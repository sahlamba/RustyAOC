use std::fs::File;
use std::io::{self, BufRead, BufReader};

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let file = File::open("src/2023/day03/input.txt")?;
    let reader = BufReader::new(file);

    let grid: Vec<String> = reader.lines().collect::<Result<_, _>>()?;
    let sum = part_number_sum(grid);

    println!("Sum of part numbers: {}", sum.unwrap());
    Ok(())
}

fn part_number_sum(grid: Vec<String>) -> Result<i32, Box<dyn std::error::Error>> {
	let height = grid.len();
    let width = grid[0].len();

	let mut sum = 0;	
    for (row, line) in grid.iter().enumerate() {
        let mut col = 0;
        
        while col < width {
            let c = line.chars().nth(col).ok_or(io::Error::new(io::ErrorKind::Other, "Index out of bounds"))?;

			if !c.is_ascii_digit() {
				col += 1;
				continue;
			}
			
			let start_col = col;
			let mut number = c.to_digit(10).unwrap() as i32;
			col += 1;

			while col < width {
				let next_c = line.chars().nth(col).ok_or(io::Error::new(io::ErrorKind::Other, "Index out of bounds"))?;

				if next_c.is_ascii_digit() {
					number = number * 10 + next_c.to_digit(10).unwrap() as i32;
					col += 1;
				} else {
					break;
				}
			}
			
			let end_col = col - 1;
			
			sum += neighbor_check(&grid, height, width, row, start_col, end_col, number)?;
        }
	}

	Ok(sum)
}

fn neighbor_check(grid: &Vec<String>, height: usize, width: usize, row: usize, start_col: usize, end_col: usize, number: i32) -> Result<i32, Box<dyn std::error::Error>> {
	let row_start = if row > 0 { row - 1 } else { 0 };
	let row_end = if row < height - 1 { row + 1 } else { row };
	let col_start = if start_col > 0 { start_col - 1 } else { 0 };
	let col_end = if end_col < width - 1 { end_col + 1 } else { end_col };

	let mut is_part_number = false;
	for r in row_start..=row_end {
		let mut col = col_start;
		while col <= col_end && !is_part_number {
			let c = grid[r].chars().nth(col).ok_or(io::Error::new(io::ErrorKind::Other, "Index out of bounds"))?;
			if !c.is_ascii_digit() && c != '.' {
				is_part_number = true;
			}
			col += 1;
		}
		if is_part_number {
			break;
		}
	}

	if is_part_number {
		return Ok(number)
	}

	Ok(0)
}