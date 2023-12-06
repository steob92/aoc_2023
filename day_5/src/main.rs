// Use fs to read in file
use std::fs::read_to_string;
// Use env to get the command line arguments
use std::env;
use rayon::prelude::*;
use std::ops::Range;

#[derive(Clone,Debug)]
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

fn get_min_location( seed_id_range : Range<usize>,  maps : &Vec<Vec<MapFrag>>) -> usize {

    
    let locations = seed_id_range.into_par_iter().map( |j| {
        let mut test_id = j;
        // println!("(i,j) = ({},{})", i, j);
        for map in maps.iter(){
            test_id = step_map(test_id, &map);
        }
        test_id
    }).collect::<Vec<usize>>();
    // Only return the min
    locations.into_par_iter().min().unwrap()

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



    
    // let id_1 = seed_ids[0];
    let locations : Vec::<usize> = seed_ids.clone().into_par_iter().map(|id| {
        // println!("New Entry...");
        let mut id_parsed = id;
        for map in maps.iter(){
            id_parsed = step_map(id_parsed, &map);
            // println!("\t{} => {}", test_id, id_parsed);
        }
        id_parsed
    }).collect::<Vec<usize>>();
    
    // Locations consumed by into_par_iter
    let min_dist = locations.into_par_iter().min().unwrap();
    println!("Minimum Distance = {}", min_dist);


    // Part two
    // Loop over len (seeds/2) loop again over seeds[i*2] -> seeds[i*2 +1]
    // Vector needs to be variable length
    let mut locations_part_deux : Vec<usize> = vec![0_usize; seed_ids.len()/2];

    for i in 0..seed_ids.len()/2{
        // println!("New Entry...{} ->{}, {}", seed_ids[2*i], (seed_ids[2*i] + seed_ids[2*i+1]), seed_ids[2*i+1]);

        locations_part_deux[i] = get_min_location(
                seed_ids[2*i]..(seed_ids[2*i] + seed_ids[2*i+1]),
                &maps
            );
    }

    locations_part_deux.par_sort();
    let min_dist_part_deux = locations_part_deux[0];
    // println!("{:?}", locations_part_deux);
    println!("Minimum Distance = {}", min_dist_part_deux);

}
