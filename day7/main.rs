use itertools::Itertools;
use std::fs;
use std::iter::zip;
use std::iter::Iterator;

fn main() {
    let file_path = "input.txt";
    // let file_path = "example.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let hands: Vec<_> = contents
        .lines()
        .map(|l| l.split(" ").collect::<Vec<_>>())
        .map(|s| {
            (
                s[0].chars()
                    .map(|c| card_type(&c.to_string()))
                    .collect::<Vec<u64>>(),
                s[1].parse::<u64>().unwrap(),
            )
        })
        .collect();
    // println!("Hands: {hands:?}");
    let mut strengths: Vec<_> = hands.iter().map(|(h,s)| (hand_strength(h),s)).collect();
    // println!("Strengths: {strengths:?}");
    strengths.sort_by(|(a_0, _), (b_0, _)| a_0.partial_cmp(b_0).unwrap());
    // println!("Sorted Strengths: {strengths:?}");
    let mut winnings: u64 = 0;
    let mut rank: u64 = 1;
    for (_, b) in strengths.iter(){
        winnings += *b * rank;
        rank += 1;
    }
    println!("Result: {winnings}");

}

fn hand_strength(hand: &Vec<u64>) -> u64 {
    // Create scoring type * 13 ** 5
    // and then card_value * 13**(4 - n) where n is the position of the card in the hand -> we can give every hand a unique score
    let position_strength: [u64; 5] = [u64::pow(13, 4), u64::pow(13, 3), u64::pow(13, 2), 13, 1];
    let h_type = hand_type(hand);
    let strength = h_type * u64::pow(13, 5)
        + zip(hand, position_strength)
            .map(|(h, p)| h * p)
            .collect::<Vec<_>>()
            .iter()
            .sum::<u64>();
    return strength;
}

fn card_type(card: &str) -> u64 {
    match card {
        "A" => return 12,
        "K" => return 11,
        "Q" => return 10,
        "J" => return 9,
        "T" => return 8,
        "9" => return 7,
        "8" => return 6,
        "7" => return 5,
        "6" => return 4,
        "5" => return 3,
        "4" => return 2,
        "3" => return 1,
        "2" => return 0,
        &_ => return 0,
    }
}
fn hand_type(hand: &Vec<u64>) -> u64 {
    let eval = hand.iter().unique().count();
    if eval == 1 {
        return 6;
    } else if eval == 5 {
        return 0;
    } else if eval == 4 {
        return 1;
    } else if eval == 2 {
        // Could be four of a kind or full house
        let eval2 = hand.iter().filter(|&n| *n == hand[0]).count();
        if eval2 == 1 || eval2 == 4 {
            return 5;
        }
        return 4;
    } else if eval == 3 {
        // Could be two pair or three of a kind
        let eval2 = hand.iter().filter(|&n| *n == hand[0]).count();
        let eval3 = hand.iter().filter(|&n| *n == hand[1]).count();

        if eval2 == 3 || eval3 == 3 || (eval2 == 1 && eval3 == 1) {
            return 3;
        }
        return 2;
    } else {
        return 0;
    }
}
