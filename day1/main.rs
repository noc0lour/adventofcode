use std::fs;
use std::iter::Iterator;

fn main() {
    let file_path = "input.txt";
    // let file_path = "example.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    // let mut total_value: u32 = 0;
    // for line in contents.lines() {
    //     let nums: Vec<&str> = line.matches(char::is_numeric).collect();
    //     let value: u32 = vec![
    //         nums.first().copied().unwrap(),
    //         nums.last().copied().unwrap(),
    //     ]
    //         .join("")
    //         .parse()
    //         .unwrap();
    //     total_value += value;
    // }

    // println!("Result: {total_value}");
    let replacements = &[
        // &["zero", ""],
        &["one", "o1e"],
        &["two", "t2o"],
        &["three", "t3e"],
        &["four", "f4r"],
        &["five", "f5e"],
        &["six", "s6x"],
        &["seven", "s7n"],
        &["eight", "e8t"],
        &["nine", "n9e"]
    ];

    let mut new_lines: Vec<String> = contents.lines().map(|line| line.to_string()).collect();
    for val in replacements.iter().rev() {
       new_lines = new_lines.iter().map(|line| line.replace(val[0], val[1])).collect();
    }
    let mut new_total_value: u64 = 0;
    for line in new_lines {
        println!("{line}");
        let nums: Vec<&str> = line.matches(char::is_numeric).collect();
        let value: u64 = vec![
            nums.first().copied().unwrap(),
            nums.last().copied().unwrap(),
        ]
            .join("")
            .parse()
            .unwrap();
        new_total_value += value;
        println!("{value}");
        println!("Running total: {new_total_value}");
    }
    println!("New Result: {new_total_value}");
}
