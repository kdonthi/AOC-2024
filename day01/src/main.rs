use std::fs;
use std::ops::Index;

const FILE_PATH: &str = "./input.txt";

fn main() {
    let (list1, list2) = lists();
    let list1 = sort_vec(list1.clone());
    let list2 = sort_vec(list2.clone());

    let mut total = 0;
    for (ind, elem_1) in list1.iter().enumerate() {
        let elem_2 = list2.get(ind).unwrap();
        println!("{} {}", elem_1, elem_2);
        let diff = elem_1 - elem_2;
        total += diff.abs();
    }

    println!("diff {}", total);
}

fn lists() -> (Vec<i32>, Vec<i32>) {
    let input_data: Vec<u8> = fs::read(FILE_PATH).unwrap();
    let input_str = String::from_utf8(input_data).unwrap();
    let mut lines = input_str.split("\n");
    println!("{}", input_str);

    let mut vec1 = Vec::new();
    let mut vec2 = Vec::new();

    loop {
        if let Some(line) = lines.next() {
            let line_elems: Vec<&str> = line.split("   ").collect();
            let first = line_elems.get(0).unwrap().parse::<i32>().unwrap();
            vec1.push(first);

            let second = line_elems.get(1).unwrap().parse::<i32>().unwrap();
            vec2.push(second);
        } else {
            break
        }
    }

    return (vec1, vec2);
}

fn sort_vec(mut v: Vec<i32>) -> Vec<i32> {
    for _i in (0..v.len()) {
        for j in (0..v.len() - 1) {
            if v[j] > v[j + 1] {
                let temp = v[j];
                v[j] = v[j + 1];
                v[j + 1] = temp;
            }
        }
    }

    return v;
}
