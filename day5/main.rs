#![feature(iter_array_chunks)]

use std::fs;
use std::iter::Iterator;
use rayon::prelude::*;
// use array_tool::vec::Intersect;

fn main() {
    let file_path = "input.txt";
    // let file_path = "example.txt";
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut lines = contents.lines();
    let seeds = lines.next().unwrap().split(":").collect::<Vec<_>>().get(1).unwrap().trim().split(" ").filter_map(|n| n.parse::<usize>().ok()).collect::<Vec<_>>();

    let mut maps: Vec<Vec<Vec<usize>>> = vec![];

    for line in lines {
        if line == "" { continue };
        if line.find("map").is_some() { maps.push(vec![]); continue};
        let mapping = line.split(" ").filter_map(|n| n.parse::<usize>().ok()).collect();
        let maps_len = maps.len();
        maps[maps_len - 1].push(mapping);
    }
    let locations: Vec<_> = seeds.iter().map(|s| remap_seed(&maps, s)).collect();
    let min_location = locations.iter().min().unwrap();
    println!("Result: {min_location}");

    let seed_iterators = seeds.iter().array_chunks::<2>().map(|a| *a[0]..(a[0]+a[1]+1)).collect::<Vec<_>>();
    let new_min_location = seed_iterators.par_iter().flat_map(|it| it.clone()).map(|s| remap_seed(&maps, &s)).min().unwrap();
    println!("New Result: {new_min_location}");
}

fn remap_seed(mappings: &Vec<Vec<Vec<usize>>>, seed_id: &usize) -> usize{
    let mut curr_id = *seed_id;
    '_map: for map in mappings.iter(){
        // println!("{curr_id}");
        for remap in map.iter(){
            if curr_id >= remap[1] && curr_id < remap[1] + remap[2]{
                curr_id = remap[0] + (curr_id - remap[1]);
                continue '_map;
            }
        }
    }
    return curr_id;
}
