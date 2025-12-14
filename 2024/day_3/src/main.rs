use std::fs;

fn multiply_numbers(str: &String, index: usize) -> Option<i32> {
    let chars_iter = str.chars().skip(index + 4);

    let mut num1 = 0;
    let mut num2 = 0;

    let mut num_count = 0;
    let mut num1_filled = false;

    for c in chars_iter {
        if c == ',' {
            // The first number is not yet filled
            if num_count == 0 {
                return None;
            }

            // Reset flags
            num_count = 0;
            num1_filled = true;
        }
        else if c.is_numeric() {
            // Return early as the number is more than 3 digits
            if num_count == 3 {
                return None;
            }
            num_count += 1;
            if !num1_filled {
                num1 *= 10;
                num1 += c.to_string().parse::<i32>().unwrap();
            } else {
                num2 *= 10;
                num2 += c.to_string().parse::<i32>().unwrap();
            }
            continue;
        }
        else if c == ')' {
            break;
        }
        else {
            return None;
        }
    }
    println!("{num1} * {num2}");
    Some(num1*num2)
}

fn sum_valid_muls(mut input: String) -> i32 {
    let mut sum = 0;
    while let Some(index) = input.rfind("mul") {
        if let Some(res) = multiply_numbers(&input, index) {
            sum += res;
        }
        input.truncate(index);
    }

    sum
}

fn sum_valid_muls_part2(mut input: String) -> i32 {
    let mut sum = 0;
    while let Some(index) = input.rfind("mul") {
        let do_index = input.rfind("do()");
        let dont_index = input.rfind("don't()");

        // truncate if do or dont is in front of our mul
        if do_index > Some(index) {
            input.truncate(do_index.unwrap());
            continue;
        }
        if dont_index > Some(index) {
            input.truncate(dont_index.unwrap());
            continue;
        }

        println!("dont: {dont_index:?} do: {do_index:?}");
        if do_index.is_none() && dont_index.is_some() {
            println!("Skipp, no do");
            input.truncate(index);
            continue;
        } else if do_index < dont_index {
            println!("Skipp do is less than dont");
            input.truncate(index);
            continue;
        }

        if let Some(res) = multiply_numbers(&input, index) {
            sum += res;
        }
        input.truncate(index);
    }

    sum
}

fn main() {
    println!("{}", sum_valid_muls("anjay mabar don't() mul(2,2)".to_string()));
    println!("Testing\nPart 1: {}", sum_valid_muls(fs::read_to_string("src/test.txt").expect("Cant't read the test file")));

    println!("Real Input\nPart 1: {}", sum_valid_muls(fs::read_to_string("src/input.txt").expect("Cant't read the input file")));

    println!("{}", sum_valid_muls_part2("anjay mabar don't() mul(2,2)".to_string()));
    println!("Testing\nPart 2: {}", sum_valid_muls_part2(fs::read_to_string("src/test2.txt").expect("Cant't read the test file")));

    println!("Real Input\nPart 2: {}", sum_valid_muls_part2(fs::read_to_string("src/input.txt").expect("Cant't read the input file")));
}
