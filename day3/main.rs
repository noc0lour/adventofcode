use regex::Regex;
use std::fs;
use std::iter::Iterator;

struct PNum {
    x: usize,
    y: usize,
    len: usize,
    num: u32,
}

struct StarItem {
    x: usize,
    y: usize,
    num_id: usize,
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
    let mut star_map: Vec<StarItem> = vec![];
    '_num_loop: for (num_idx, num) in all_nums.iter().enumerate() {
        // Check if any of the surrounding symbols is not a '.' and not a number
        let x_num: i32 = num.x as i32;
        let y_num: i32 = num.y as i32;
        // Start coordinates are left above of num
        let x: i32 = x_num - 1;
        let y: i32 = y_num - 1;
        let mut num_recorded: bool = false;
        // Check above
        for y_shift in 0..3 {
            if y + (y_shift as i32) >= 0 && y + (y_shift as i32) < max_y as i32 {
                for x_shift in 0..(num.len + 2) {
                    if (x + x_shift as i32) >= 0 && (x + x_shift as i32) < max_x as i32 {
                        if !num_recorded {
                            if check_pos(
                                &lines,
                                (x + x_shift as i32) as usize,
                                (y + y_shift as i32) as usize,
                            ) {
                                sum_of_all += num.num;
                                num_recorded = true;
                            }
                        }
                        if check_star(
                            &lines,
                            (x + x_shift as i32) as usize,
                            (y + y_shift as i32) as usize,
                        ) {
                            star_map.push(StarItem {
                                x: (x + x_shift as i32) as usize,
                                y: (y + y_shift as i32) as usize,
                                num_id: num_idx,
                            });
                        }
                    }
                }
            }
        }
    }
    let mut visited_stars: Vec<[usize; 2]> = vec![];
    let mut sum_products: u32 = 0;
    for star in star_map.iter(){
        if visited_stars.iter().find(|s| s[0] == star.x && s[1] == star.y).is_some(){
            continue;
        }
        let other_items: Vec<&StarItem> = star_map.iter().filter(|s| s.x == star.x && s.y == star.y).collect();
        if other_items.len() > 1{
            let product: u32 = other_items.iter().map(|i| all_nums[i.num_id].num).product();
            sum_products += product;
        }
        visited_stars.push([star.x, star.y]);
    }
    println!("Result: {sum_of_all}");
    println!("Sum of Products: {sum_products}");
}

fn check_pos(lines: &Vec<String>, x: usize, y: usize) -> bool {
    return !(lines[y].chars().nth(x).unwrap() == '.')
        && !lines[y].chars().nth(x).unwrap().is_numeric();
}

fn check_star(lines: &Vec<String>, x: usize, y: usize) -> bool {
    return lines[y].chars().nth(x).unwrap() == '*';
}
