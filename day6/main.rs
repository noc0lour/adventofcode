use std::fs;
use std::iter::Iterator;
use std::iter::zip;

fn main(){
    let file_path = "input.txt";
    // let file_path = "example.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut lines = contents.lines();
    let times = lines.next().unwrap().split(":").collect::<Vec<_>>().get(1).unwrap().trim().split(" ").filter_map(|n| n.parse::<u64>().ok()).collect::<Vec<_>>();
    let distances = lines.next().unwrap().split(":").collect::<Vec<_>>().get(1).unwrap().trim().split(" ").filter_map(|n| n.parse::<u64>().ok()).collect::<Vec<_>>();
    let product_solutions: u64 = zip(times.iter(), distances.iter()).map(|(t, d)| time_distance_solutions(t, d)).product();
    println!("Result: {product_solutions}");

    let time = times.iter().map(|n| n.to_string()).collect::<Vec<_>>().join("").parse::<u64>().unwrap();
    let distance = distances.iter().map(|n| n.to_string()).collect::<Vec<_>>().join("").parse::<u64>().unwrap();
    let solution: u64 = time_distance_solutions(&time, &distance);
    println!("New Result: {solution}");
}

fn time_distance_solutions(time: &u64, win_distance: &u64) -> u64{
    let sol_1 = (0.5 * *time as f32 + f32::sqrt(0.25 * f32::powi(*time as f32, 2) - *win_distance as f32) - 0.001).floor() as u64;
    let sol_2 = (0.5 * *time as f32 - f32::sqrt(0.25 * f32::powi(*time as f32, 2) - *win_distance as f32) + 0.001).ceil() as u64;
    let possible_solutions = sol_1 - sol_2 + 1;
    println!("sol: {sol_1}, {sol_2}");
    return possible_solutions;
}
