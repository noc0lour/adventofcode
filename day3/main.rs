use regex::Regex;
use std::fs;
use std::iter::Iterator;

struct PNum {
    x: usize,
    y: usize,
    len: usize,
    num: u32,
}

fn main() {
    let file_path = "input.txt";
    // let file_path = "example.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let num_re = Regex::new(r"(?m)(\d+)").unwrap();

    let lines: Vec<String> = contents.lines().map(|line| line.to_string()).collect();
    let max_y = lines.len();
    let max_x = lines[0].len();
    let mut all_nums: Vec<PNum> = vec![];
    for (line_id, line) in lines.iter().enumerate() {
        // println!("Line {line_id}");
        for cap in num_re.captures_iter(&line).filter(|c| c.len() > 1) {
            let start_idx = cap.get(1).unwrap().start();
            let (_, [num]) = cap.extract();
            all_nums.push(PNum {
                x: start_idx,
                y: line_id,
                len: num.len(),
                num: num.parse::<u32>().unwrap(),
            });
        }
    }
    let all_nums_len = all_nums.len();
    println!("Number of numbers: {all_nums_len}");

    let mut sum_of_all: u32 = 0;
    '_num_loop: for num in all_nums.iter() {
        // Check if any of the surrounding symbols is not a '.' and not a number
        let x_num: i32 = num.x as i32;
        let y_num: i32 = num.y as i32;
        // Start coordinates are left above of num
        let x: i32 = x_num - 1;
        let y: i32 = y_num - 1;
        // Check above
        for y_shift in 0..3 {
            if y + (y_shift as i32) >= 0 && y + (y_shift as i32) < max_y as i32 {
                for x_shift in 0..(num.len + 2) {
                    if (x + x_shift as i32) >= 0 && (x  + x_shift as i32) < max_x as i32 {
                        if check_pos(
                            &lines,
                            (x + x_shift as i32) as usize,
                            (y + y_shift as i32) as usize,
                        ) {
                            sum_of_all += num.num;
                            continue '_num_loop;
                        }
                    }
                }
            }
        }
    }
    println!("Result: {sum_of_all}");
}

fn check_pos(lines: &Vec<String>, x: usize, y: usize) -> bool {
    return !(lines[y].chars().nth(x).unwrap() == '.')
        && !lines[y].chars().nth(x).unwrap().is_numeric();
}
