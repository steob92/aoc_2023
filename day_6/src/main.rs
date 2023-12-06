// Use fs to read in file
use std::fs::read_to_string;
// Use env to get the command line arguments
use std::env;
use rayon::prelude::*;


// could use calculus...
// fn get_max_speed(duration: usize ) -> (f32, f32) {
//     // r = ut + 0.5*at^2 -> a = 0
//     // r = ut  -?> u = press; t = (duration - press)
//     // r = press(duration - press) -> press x duration - press^2
//     // dr/dpress = duration - 2 press
//     // max -> press = 0.5 duration
//     let press = 0.5 * duration as f32;
//     (press, (press) * (duration as f32- press) )
// }

// Quicker to just code it
fn get_distance(press: usize, duration: usize) -> usize{
    press * (duration - press)
}


fn main() {


    let args  = env::args().collect::<Vec<String>>();

    // Read in a text file
    let filename = &args[1];

    // Parse lines to a vec of strings
    let my_vec = read_to_string(filename).unwrap().lines().map(|x| x.to_string()).collect::<Vec<String>>();

    let time = my_vec[0].split(":")
                        .collect::<Vec<_>>()[1]
                        .split_ascii_whitespace()
                        .map(|x| x.parse::<usize>().unwrap())
                        .collect::<Vec<usize>>();
    let distance = my_vec[1].split(":")
                        .collect::<Vec<_>>()[1]
                        .split_ascii_whitespace()
                        .map(|x| x.parse::<usize>()
                        .unwrap())
                        .collect::<Vec<usize>>();

    // let max_speeds = time.iter().map(|&t| get_max_speed(t)).collect::<Vec<(f32, f32)>>(); 
    // let roots = time.iter().zip(distance.iter()).map(|(&t, &d)| get_roots(t, d)).collect::<Vec<(f32, f32)>>(); 
    
    // println!("{:?}", time);
    // println!("{:?}", distance);
    // println!("{:?}", roots);

    let race_sum = (0..time.len()).into_par_iter().map(|i| {
        (0..time[i])
            .map(|x| get_distance(x, time[i]))
            .collect::<Vec<usize>>()
            .iter()
            .filter(|&&x| x > distance[i])
            .map(|_| 1)
            .sum()
    }).collect::<Vec<usize>>();

    println!("Total sum: {}", race_sum.iter().product::<usize>());


    // part 2
    // Real race time
    let mut total_time :String  = String::from(""); 
    let mut total_distance :String  = String::from(""); 
    (0..time.len()).into_iter().for_each(|i| {
        total_time += &time[i].to_string();
        total_distance += &distance[i].to_string();
    });

    let total_time = total_time.parse::<usize>().unwrap();
    let total_distance = total_distance.parse::<usize>().unwrap();
    
    println!("Total Time {}", total_time);
    println!("Total Distance {}", total_distance);

    let part_two_total = (0..total_time).into_par_iter()
        .map(|x| get_distance(x, total_time))
        .collect::<Vec<usize>>()
        .iter()
        .filter(|&&x| x > total_distance)
        .map(|_| 1)
        .sum::<usize>();

    println!("Total Wins {}", part_two_total);

}
