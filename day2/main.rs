use std::fs;
use std::iter::Iterator;
use regex::Regex;


fn main(){
    let file_path = "input.txt";
    // let file_path = "example.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let red_cubes = 12;
    let green_cubes = 13;
    let blue_cubes = 14;

    let line_re = Regex::new(r"(?m)^Game (\d+): (.+)$").unwrap();
    // let mut results = vec![];
    // println!("{contents}");
    let mut total_id: u32 = 0;
    let mut total_id_possible: u32 = 0;
    for (_, [game_id, game_content]) in line_re.captures_iter(&contents).map(|c| c.extract()){
        // println!("Game ID: {game_id}, Game Content: {game_content}");
        total_id_possible += game_id.parse::<u32>().unwrap();
        let games = game_content.split(";");
        '_game: for game in games{
            for color in game.split(",").map(|c| c.trim()){
                let num: u32 = color.matches(char::is_numeric).collect::<Vec<&str>>().join("").parse().unwrap();
                if color.contains("blue"){
                    if num > blue_cubes{
                        total_id += game_id.parse::<u32>().unwrap();
                        // println!("id: {game_id}");
                        break '_game;
                    }
                } else if color.contains("red"){
                    if num > red_cubes{
                        total_id += game_id.parse::<u32>().unwrap();
                        // println!("id: {game_id}");
                        break '_game;
                    }
                } else if color.contains("green"){
                    if num > green_cubes{
                        total_id += game_id.parse::<u32>().unwrap();
                        // println!("id: {game_id}");
                        break '_game;
                    }
                }
            }
        }
    }
    let actual_result = total_id_possible - total_id;
    println!("Result: {actual_result}");
    // for line in contents.lines(){
    //     let game_id: u32 = line.match("Game ")
    // }
    let mut total_power: u32 = 0;
    for (_, [game_id, game_content]) in line_re.captures_iter(&contents).map(|c| c.extract()){
        // println!("Game ID: {game_id}, Game Content: {game_content}");
        total_id_possible += game_id.parse::<u32>().unwrap();
        let mut min_cubes = vec![0, 0, 0]; // red green blue
        let games = game_content.split(";");
        '_game: for game in games{
            for color in game.split(",").map(|c| c.trim()){
                let num: u32 = color.matches(char::is_numeric).collect::<Vec<&str>>().join("").parse().unwrap();
                if color.contains("blue"){
                    if num > min_cubes[2]{
                        min_cubes[2] = num;
                    }
                } else if color.contains("red"){
                    if num > min_cubes[0]{
                        min_cubes[0] = num;
                    }
                } else if color.contains("green"){
                    if num > min_cubes[1]{
                        min_cubes[1] = num;
                    }
                }
            }
        }
        let power: u32 = min_cubes.iter().product();
        total_power += power;
    }
    println!("Sum of all powers: {total_power}");
}
