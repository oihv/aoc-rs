use std::fs;

fn count_xmas(line: &String) -> u32 {
    let mut line = line.clone();
    let mut line_clone = line.clone();

    let mut count = 0;

    while let Some(index) = line.rfind("XMAS") {
        count += 1;
        // dbg!(&line);
        line.truncate(index);
    }

    while let Some(index) = line_clone.rfind("SAMX") {
        count += 1;
        // dbg!(&line_clone);
        line_clone.truncate(index);
    }

    count
}

fn process(path: &str) -> u32 {
    let input = fs::read_to_string(path).expect("File not found");

    let mut sum = 0;
    // Check in each line
    for line in input.lines() {
        sum += count_xmas(&(line).to_string());
    }
    // Convert vector for easy access
    let mut input_vec: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let mut line_vec: Vec<char> = Vec::new();
        for char in line.chars() {
            line_vec.push(char);
        }
        input_vec.push(line_vec);
    }

    // Get the dimension of the input
    let width = input_vec[0].len();
    let height = input_vec.len();

    // Check vertical
    for i in 0..width {
        let mut vert_line = String::new();
        for j in 0..height {
            vert_line.push(input_vec[j][i]);
        }
        sum += count_xmas(&vert_line);
    }
    // Check diagonal (top left - bottom right)
    for i in 1..height-3 {
        let mut diagonal_line = String::new();
        let mut inc = 0;
        while i+inc < height && inc < width {
            diagonal_line.push(input_vec[i+inc][inc]);
            inc += 1;
        }
        sum += count_xmas(&diagonal_line);
        // dbg!(&diagonal_line);
    }
    for i in 0..width-3 {
        let mut diagonal_line = String::new();
        let mut inc = 0;
        while i+inc < width && inc < height {
            diagonal_line.push(input_vec[inc][i+inc]);
            inc += 1;
        }
        sum += count_xmas(&diagonal_line);
        // dbg!(&diagonal_line);
    }
    // Check diagonal (top right - bottomm left)
    for i in 3..height {
        let mut diagonal_line = String::new();
        let mut inc = 0;
        // i >= inc so that the vector index will never be 0, and the calculation didn't result in
        // overflow
        while i >= inc && inc < width {
            diagonal_line.push(input_vec[i-inc][inc]);
            inc += 1;
        }
        sum += count_xmas(&diagonal_line);
        // dbg!(&diagonal_line);
    }
    for i in 1..width-3 {
        let mut diagonal_line = String::new();
        let mut inc = 0;
        while i+inc < width && inc <= height {
            diagonal_line.push(input_vec[height-inc-1][i+inc]);
            inc += 1;
        }
        sum += count_xmas(&diagonal_line);
        // dbg!(&diagonal_line);
    }
    sum
}

fn get_mas_index(line: &String) -> Vec<usize> {
    let mut line = line.clone();
    let mut line_clone = line.clone();

    let mut res = Vec::new();

    while let Some(index) = line.rfind("MAS") {
        // dbg!(&line);
        res.push(index+1);
        line.truncate(index);
    }

    while let Some(index) = line_clone.rfind("SAM") {
        // dbg!(&line_clone);
        res.push(index+1);
        line_clone.truncate(index);
    }

    res
}

fn process_part2(path: &str) -> u32 {
    let input = fs::read_to_string(path).expect("File not found");

    let mut sum = 0;
    // Convert vector for easy access
    let mut input_vec: Vec<Vec<char>> = Vec::new();
    for line in input.lines() {
        let mut line_vec: Vec<char> = Vec::new();
        for char in line.chars() {
            line_vec.push(char);
        }
        input_vec.push(line_vec);
    }

    // Get the dimension of the input
    let width = input_vec[0].len();
    let height = input_vec.len();

    // Mas mark for possible xmas spots
    let mut mas_map: Vec<(usize, usize)> = Vec::new();

    // Check diagonal (top left - bottom right)
    for i in 1..height-2 {
        let mut diagonal_line = String::new();
        let mut inc = 0;
        while i+inc < height && inc < width {
            diagonal_line.push(input_vec[i+inc][inc]);
            inc += 1;
        }
        for j in get_mas_index(&diagonal_line) {
            mas_map.push((i+j, j));
        }
        // dbg!(&diagonal_line);
    }
    for i in 0..width-2 {
        let mut diagonal_line = String::new();
        let mut inc = 0;
        while i+inc < width && inc < height {
            diagonal_line.push(input_vec[inc][i+inc]);
            inc += 1;
        }
        for j in get_mas_index(&diagonal_line) {
            mas_map.push((j, i+j));
        }
        // dbg!(&diagonal_line);
    }
    // Check diagonal (top right - bottomm left)
    for i in 2..height {
        let mut diagonal_line = String::new();
        let mut inc = 0;
        // i >= inc so that the vector index will never be 0, and the calculation didn't result in
        // overflow
        while i >= inc && inc < width {
            diagonal_line.push(input_vec[i-inc][inc]);
            inc += 1;
        }
        for j in get_mas_index(&diagonal_line) {
            if mas_map.contains(&(i-j, j)) {
                sum += 1;
            }
        }
        // dbg!(&diagonal_line);
    }
    for i in 1..width-2 {
        let mut diagonal_line = String::new();
        let mut inc = 0;
        while i+inc < width && inc <= height {
            diagonal_line.push(input_vec[height-inc-1][i+inc]);
            inc += 1;
        }
        for j in get_mas_index(&diagonal_line) {
            if mas_map.contains(&(height-j-1, i+j)) {
                sum += 1;
            }
        }
        // dbg!(&diagonal_line);
    }
    sum
}
fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn part1_test_input() {
        assert_eq!(process("src/test.txt"), 18);
    }
    #[test]
    fn part1_real_input() {
        assert_eq!(process("src/input.txt"), 2536);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(process_part2("src/test.txt"), 9);
    }
    #[test]
    fn part2_real_input() {
        assert_eq!(process_part2("src/input.txt"), 2536);
    }
}

