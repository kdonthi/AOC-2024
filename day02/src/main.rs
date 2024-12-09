const FILE_PATH: &str = "./input.txt";

fn main() {
    let file_txt = std::fs::read_to_string(FILE_PATH)
        .expect(format!("not able to read from {}", FILE_PATH).as_str());

    let lines: Vec<&str> = file_txt.split("\n").collect();
    let lines_count = lines.len();

    let mut incorrect_count = 0;
    for (ind_1, line) in lines.iter().enumerate() {
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

