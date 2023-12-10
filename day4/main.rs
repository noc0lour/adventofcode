use regex::Regex;
use std::fs;
use std::iter::Iterator;
// use array_tool::vec::Intersect;

fn main() {
    let file_path = "input.txt";
    // let file_path = "example.txt";

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let line_re = Regex::new(r"(?m)^Card\s+(\d+):(.*)\|(.*)$").unwrap();
    let mut total_points: u32 = 0;
    let mut card_points: Vec<usize> = vec![0; contents.lines().count()];
    let mut num_cards: usize = contents.lines().count();
    for (_, [card_id, winning_numbers, my_numbers]) in
        line_re.captures_iter(&contents).map(|c| c.extract())
    {
        let wn: Vec<u32> = winning_numbers
            .trim()
            .split(" ")
            .filter_map(|n| n.parse::<u32>().ok())
            .collect();
        let mn: Vec<u32> = my_numbers
            .trim()
            .split(" ")
            .filter_map(|n| n.parse::<u32>().ok())
            .collect();
        let number_wins = mn
            .iter()
            .filter_map(|n| wn.iter().find(|m| *m == n))
            .count();
        card_points[card_id.parse::<usize>().unwrap() - 1] = number_wins;
        if number_wins > 0 {
            let points = u32::pow(2, number_wins as u32 - 1);
            total_points += points;
        }
    }
    println!("Result: {total_points}");
    // println!("Card points: {card_points:?}");
    let mut card_ids: Vec<usize> = (0..num_cards).collect();
    while card_ids.iter().count() > 0 {
        // println!("card ids: {card_ids:?}");
        card_ids = card_ids
            .iter()
            .map(|c| (c, card_points[*c]))
            .map(|(id, points)| (id + 1)..(id + points + 1))
            .collect::<Vec<_>>()
            .iter()
            .flat_map(|it| it.clone())
            .collect();
        num_cards += card_ids.iter().count();
    }
    println!("Num cards: {num_cards}");
}
