use std::fs;

#[derive(PartialEq)]
enum ReportTrend {
    Increasing,
    Decreasing,
    Flat
}

fn get_report_trend(prev: &i32, curr: &i32) -> ReportTrend {
    if prev > curr {
        ReportTrend::Decreasing
    } else if curr > prev {
        ReportTrend::Increasing
    } else {
        ReportTrend::Flat
    }
}

fn get_report_disparity(prev: &i32, curr: &i32) -> u32 {
    prev.abs_diff(*curr)
}

fn check_report_safety(report: &Vec<i32>) -> bool {
    let mut numbers = report.iter();

    let mut prev = numbers.next().unwrap();

    let curr = numbers.next().unwrap();

    let trend = get_report_trend(prev, curr);

    if trend == ReportTrend::Flat {
        return false;
    }

    let disparity = get_report_disparity(prev, curr);
    if !(1..=3).contains(&disparity) {
        return false;
    }
    
    prev = curr;

    for curr in numbers {
        if get_report_trend(prev, curr) != trend {
            return false;
        }  

        let disparity = get_report_disparity(prev, curr);
        if !(1..=3).contains(&disparity) {
            return false;
        }

        prev = curr;
    } 
    true
}

fn check_report_safety_fallback(nums: Vec<i32>) -> bool {
    for i in 0..nums.len() {
        let mut nums = nums.clone();
        nums.remove(i);

        if check_report_safety(&nums) {
            return true
        }
    }
    false
    
}

fn get_safe_report_sum(data: String, fallback: bool) -> u32 {
    let mut reports = data.lines();
    let mut sum = 0;

    while let Some(line) = reports.next() {
        let numbers = line.split(" ").map(|s| s.parse::<i32>().unwrap());
        let numbers_vec: Vec<i32> = numbers.clone().collect();
        if check_report_safety(&numbers_vec) || fallback && check_report_safety_fallback(numbers_vec) {
            sum += 1;
        }
    }
    sum
}

fn main() {
    println!("Testing\nPart 1: {}", get_safe_report_sum(fs::read_to_string("src/test.txt").expect("Cant't read the test file"), false));

    println!("Real Input\nPart 1: {}", get_safe_report_sum(fs::read_to_string("src/input.txt").expect("Cant't read the input file"), false));

    println!("Testing\nPart 2: {}", get_safe_report_sum(fs::read_to_string("src/test.txt").expect("Cant't read the test file"), true));

    println!("Real Input\nPart 2: {}", get_safe_report_sum(fs::read_to_string("src/input.txt").expect("Cant't read the input file"), true));
}
