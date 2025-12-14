#[derive(Debug)]
enum Direction {
    Left,
    Right,
}

#[derive(Debug)]
struct Rotation {
    dir: Direction,
    mag: i32,
}

fn get_rotation(command: &str) -> Rotation {
    let mut res = Rotation {
        dir: Direction::Left,
        mag: 0,
    };

    match command.chars().next() {
        Some('L') => res.dir = Direction::Left,
        Some('R') => res.dir = Direction::Right,
        _ => panic!("Command direction is not right!"),
    }

    let num = command
        .strip_prefix(|ch| ch == 'L' || ch == 'R')
        .expect("Direction not found");
    res.mag = num.parse().expect("Magnitude not correct");
    res
}

fn part1(input: &str) -> i32 {
    let mut count = 0;
    let lines = input.lines();
    let mut pos = 50;

    for line in lines {
        let mut mul = 0;
        let rotation = get_rotation(line);
        match rotation.dir {
            Direction::Left => mul = -1,
            Direction::Right => mul = 1,
        }

        pos += mul * rotation.mag;

        while pos >= 100 {
            pos -= 100;
        }
        while pos < 0 {
            pos += 100;
        }
        // println!("{pos}");

        if pos == 0 {
            count += 1;
        }
    }
    count
}

fn part2(input: &str) -> i32 {
    let mut count = 0;
    let lines = input.lines();
    let mut pos = 50;
    let mut prev_zero = false;

    for line in lines {
        let mut mul = 0;
        let mut increased = false;
        let rotation = get_rotation(line);
        match rotation.dir {
            Direction::Left => mul = -1,
            Direction::Right => mul = 1,
        }

        pos += mul * rotation.mag;
        println!("instruction: {rotation:?}");
        println!("{pos}");

        while pos >= 100 {
            pos -= 100;
            if prev_zero {
                prev_zero = false;
                // if pos > 0 {
                //     continue;
                // }
            }
            count += 1;
            increased = true;
            println!("Zero passed!");
        }
        while pos < 0 {
            pos += 100;
            if prev_zero {
                prev_zero = false;
                continue;
            }
            count += 1;
            println!("Zero passed!");
        }
        if prev_zero {
            prev_zero = false;
        }
        if pos == 0 {
            if !increased {
                count += 1;
            }
            prev_zero = true;
            println!("Zero found!");
        }
        println!("after normalization: {pos}\n");
    }
    count
}

fn main() {
    println!("Hello AOC 25!");
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn part1_test_input() {
        assert_eq!(part1(include_str!("test.txt")), 3);
    }
    #[test]
    fn part1_real_input() {
        assert_eq!(part1(include_str!("input.txt")), 1023);
    }

    #[test]
    fn part2_test_input() {
        assert_eq!(part2(include_str!("test.txt")), 6);
    }
    #[test]
    // #[ignore]
    fn part2_real_input() {
        assert_eq!(part2(include_str!("input.txt")), 2536);
    }
    #[test]
    fn r1000() {
        assert_eq!(part2("R1000"), 10);
    }
    #[test]
    fn l1000() {
        assert_eq!(part2("L1000"), 10);
    }
    #[test]
    fn r50() {
        assert_eq!(part2("R50"), 1);
    }
    #[test]
    // #[ignore]
    fn r150() {
        assert_eq!(part2("R150"), 2);
    }
    #[test]
    // #[ignore]
    fn l50() {
        assert_eq!(part2("L50"), 1);
    }
    #[test]
    // #[ignore]
    fn l100() {
        assert_eq!(part2("L100"), 1);
    }
    #[test]
    // #[ignore]
    fn l150() {
        assert_eq!(part2("L150"), 2);
    }
    #[test]
    fn l200() {
        assert_eq!(part2("L200"), 2);
    }
    #[test]
    // #[ignore]
    fn l1050() {
        assert_eq!(part2("L1050"), 11);
    }
    #[test]
    // #[ignore]
    fn test_from_reddit() {
        assert_eq!(
            part2(
                "R1000
L149
L1
R1
L2
R1
L1
R2
R99"
            ),
            16
        );
    }
    #[test]
    // #[ignore]
    fn land_on_zero_then_swing_pass_zero() {
        assert_eq!(
            part2(
"R50
R120"
            ),
            2
        );
    }
    #[test]
    fn test_left() {
        assert_eq!(
           part2(
"R25
L100
R25
L200"
            ),
            4
        );
    }
    #[test]
    // #[ignore]
    fn double_left_full_rotate() {
        assert_eq!(part2("L250"), 3);
    }
    #[test]
    fn stay_on_zero_and_double_rotate() {
        assert_eq!(part2("L50\nL200"), 3);
    }
}
