use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    let mut right_map: HashMap<i32, i32> = HashMap::new();

    let mut lines = input.lines();

    while let Some(line) = lines.next() {
        let mut numbers = line.split("   ");

        left.push(numbers.next().unwrap().parse().unwrap());
        let curr_right = numbers.next().unwrap().parse().unwrap();
        right.push(curr_right);
        right_map.entry(curr_right).and_modify(|count| *count += 1).or_insert(1);
    }

    left.sort();
    right.sort();

    let mut sum = 0;
    for i in 0..left.len() {
        sum += (left[i] - right[i]).abs()
    }

    println!("Part one answer: {sum}");

    // Part two, finding the similarity between left and right list
    let mut similarity = 0;
    for num in left {
        similarity += num * right_map.get(&num).unwrap_or(&0);
    }

    println!("Part two answer: {similarity}");
}
