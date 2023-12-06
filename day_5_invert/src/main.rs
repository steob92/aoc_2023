// Use fs to read in file
use std::fs::read_to_string;
// Use env to get the command line arguments
use std::env;
use std::cmp::Ordering;
use std::ops::Range;
use rayon::prelude::*;
use indicatif::ParallelProgressIterator;


#[derive(Clone,Debug )]
struct MapFrag{
    dest : Range<usize>,
    source : Range<usize>,
}

fn get_seed_ids(s : &String) -> Vec<usize>{
    s.split(":")
        .collect::<Vec<_>>()[1]
        .split_ascii_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
    
}


fn step_map(id: usize, maps : &Vec<MapFrag> ) -> usize{

    for map in maps.iter(){
        if map.source.contains(&id){
            return map.dest.start + id-map.source.start;
        }
    }

    id
}

fn step_back_map(id: usize, maps : &Vec<MapFrag> ) -> usize{

        for map in maps.iter(){
            if map.dest.contains(&id){
                return map.source.start + id-map.dest.start;
            }
        }
    
        id
    }




fn brute_force_backwards(acceptable_ids : Vec<Range<usize>>, maps : &Vec<Vec<MapFrag>>) -> usize{
    let mut id :usize = 0;
    let n_core: usize = 16;
    let npar: usize = 10_000*n_core;
    loop {

        // Run in parallel batches
        let test_vec = (0..npar).into_par_iter().map(|j: usize| {
            let mut test_id: usize = id + j;

            for i in 0..maps.len(){
                let id_parsed = step_back_map(test_id, &maps[maps.len() - 1 - i]);
                // println!("\t{} => {}", test_id, id_parsed);
                test_id = id_parsed;
            }
            let mut val = 0_usize;
            for rang in acceptable_ids.iter(){
                if rang.contains(&test_id){
                    val = 1_usize;
                    break;

                }
            }
            val
        }).collect::<Vec<_>>();

        if test_vec.contains(&1){
            // println!("{:?}", test_vec);
            return test_vec.iter().position(|&r| r == 1).unwrap() + id
        } 
        id += npar;
        if id > 30_000_000{
            println!("Taking too long...");
            return id;
        }
        
    }
}

fn get_min_location( seed_id_start : usize, seed_id_stop: usize, maps : &Vec<Vec<MapFrag>>) -> usize {

    
    let mut locations = (seed_id_start..seed_id_stop).into_par_iter().map( |j| {
        
        let mut test_id = j;
        // println!("(i,j) = ({},{})", i, j);
        for map in maps.iter(){
            let id_parsed = step_map(test_id, &map);
            // println!("\t{} => {}", test_id, id_parsed);
            test_id = id_parsed;
        }
        test_id
    }).collect::<Vec<usize>>();
    // Sort
    locations.par_sort();
    // Only return the minimum
    locations[0]
}

fn main() {
    let args  = env::args().collect::<Vec<String>>();

    // Read in a text file
    let filename = &args[1];

    // Parse lines to a vec of strings
    let my_vec = read_to_string(filename).unwrap().lines().map(|x| x.to_string()).collect::<Vec<String>>();


    let seed_ids = get_seed_ids(&my_vec[0]);

    let mut maps: Vec<Vec<MapFrag>> = Vec::new();

    let mut section : Vec<MapFrag> = Vec::new();

    for i in 3..my_vec.len(){
        if my_vec[i].contains(&"map".to_string()){
            maps.push(section.clone());
            section = Vec::new();
        } else if my_vec[i].is_empty(){
            continue;
        } else {
            let tmp = my_vec[i].split_ascii_whitespace().map(|x| x.parse::<usize>().unwrap()).collect::<Vec<usize>>();
            section.push( MapFrag{
                dest : tmp[0]..(tmp[0] + tmp[2]),
                source : tmp[1]..(tmp[1] + tmp[2]),
            });

        }

    }
    // Add the last map
    maps.push(section);



    
    // let id = seed_ids[0];
    let mut test_id :usize;
    let mut locations : Vec::<usize> = vec![0_usize; seed_ids.len()];
    for (i, id) in seed_ids.iter().enumerate(){
        // println!("New Entry...");
        test_id = *id;
        for map in maps.iter(){
            let id_parsed = step_map(test_id, &map);
            // println!("\t{} => {}", test_id, id_parsed);
            test_id = id_parsed;
        }
        locations[i] = test_id;
    }


    locations.sort();
    let min_dist = locations[0];
    println!("Minimum Distance = {}", min_dist);


    // Part two
    // Loop over len (seeds/2) loop again over seeds[i*2] -> seeds[i*2 +1]
    // Vector needs to be variable length
    
    let mut acceptable_ids: Vec<Range<usize>> = Vec::new();
    for i in 0..(seed_ids.len()/2){
        acceptable_ids.push(seed_ids[i*2]..(seed_ids[i*2] + seed_ids[i*2+1]));
    }

    println!("Lenght of Vector {}", acceptable_ids.len());
    let min_dist_part_deux = brute_force_backwards(acceptable_ids, &maps);
    println!("Minimum Distance = {}", min_dist_part_deux);

}
