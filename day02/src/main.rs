use std::ops::BitAndAssign;

const FILE_PATH: &str = "./input.txt";

fn main() {
     // no_forgiveness();
    forgiveness();
}

fn forgiveness() {
    let file_txt = std::fs::read_to_string(FILE_PATH)
        .expect(format!("not able to read from {}", FILE_PATH).as_str());

    let lines: Vec<&str> = file_txt.split("\n").collect();
    let lines_count = lines.len();

    let mut incorrect_count = 0;

    let f = |first, second, decreased: bool, increased: bool| -> (bool, bool) {
        if !valid_step(first, second) {
            return (false, false);
        }

        let decreasing = (first - second > 0);
        if decreasing && !increased || !decreasing && !decreased {
            // println!("happened here !!!! {}, {}, {}", decreasing, first, second);
            return (true, decreasing);
        }
        println!("not happened here !!!! {}, {}, {}", decreasing, first, second);

        return (false, decreasing);
    };

    for (idx, line) in lines.iter().enumerate() {
        println!("{}", idx);
        let numbers: Vec<&str> = line.split(" ").collect();
        let mut decreased = false;
        let mut increased = false;

        let mut ind = 0;
        while ind < numbers.len() - 1 {
            let first = numbers[ind].parse::<i32>().unwrap();
            let second = numbers[ind + 1].parse::<i32>().unwrap();

            let mut third = -1;
            if ind < numbers.len() - 2 {
                third = numbers[ind + 2].parse::<i32>().unwrap();
            }

            let (mut valid, mut decreasing) = f(first, second, decreased, increased);
            if !valid {
                if third != -1 {
                    (valid, decreasing) = f(first, third, decreased, increased);
                    if valid {
                        ind += 1;
                    }
                    // println!("here {} {} {}", ind, first, third);
                }

                if !valid {
                    incorrect_count += 1;
                    break;
                }
            }

            if decreasing {
                decreased = true;
            } else {
                increased = true;
            }

            ind += 1;
        }
    }

    println!("result {} total {} incorrect {}", lines_count - incorrect_count, lines_count, incorrect_count);
}

fn no_forgiveness() {
    let file_txt = std::fs::read_to_string(FILE_PATH)
        .expect(format!("not able to read from {}", FILE_PATH).as_str());

    let lines: Vec<&str> = file_txt.split("\n").collect();
    let lines_count = lines.len();

    let mut incorrect_count = 0;
    for line in lines {
        let numbers: Vec<&str> = line.split(" ").collect();
        let mut decreasing = false;
        let mut increasing = false;

        for ind in 0..numbers.len()-1 {
            let first = numbers[ind].parse::<i32>().unwrap();
            let second = numbers[ind + 1].parse::<i32>().unwrap();

            let diff = first - second;
            if diff < 0 {
                increasing = true;
            } else {
                decreasing = true;
            }

            if (decreasing && increasing) || diff.abs() >= 4 || diff.abs() < 1 {
                incorrect_count += 1;
                break;
            }
        }
    }

    println!("result {} total {} incorrect {}", lines_count - incorrect_count, lines_count, incorrect_count);
}

// returns distance, and -1 if decreasing +1 if increasing/neutral
fn valid_step(first: i32, second: i32) -> (bool) {
    let diff = (first - second);
    let diff_abs = diff.abs();
    return diff_abs <= 3 && diff_abs >= 1;
}

