// Use fs to read in file
use std::fs::read_to_string;
// Use env to get the command line arguments
use std::env;

#[derive(Debug, Clone)]
struct Coord{
    x_loc_start : i32,
    x_loc_stop : i32,
    y_loc_start : i32,
    y_loc_stop : i32,
    len : i32,    
    value : String,
    numeric : bool,
}


impl Coord{

    fn new() -> Self{
        Coord{
            x_loc_start : 0,
            x_loc_stop : 0,
            y_loc_start : 0,
            y_loc_stop : 0,
            len : 0,    
            value : "".to_string(),
            numeric : false,
        }
    }


    fn is_part(self : &Self, other : &Coord) -> bool {
        if self.numeric & other.numeric {
            if (self.x_loc_start == other.x_loc_start) && ( (self.y_loc_stop + 1) == other.y_loc_start){
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn is_adjacent(self : &Self, other : &Coord) -> bool {

        // If the rows are greater than 1 away...
        if (self.x_loc_start - other.x_loc_start).abs() > 1 {
            return false;
        } 
        else if (self.y_loc_start - other.y_loc_start).abs() < 2 ||  ( self.y_loc_stop - other.y_loc_start ).abs() < 2 ||
            (self.y_loc_start - other.y_loc_stop).abs() < 2 ||  ( self.y_loc_stop - other.y_loc_stop ).abs() < 2 
        {
            // If +/- 1 from the y_loc_start/y_loc_stop  
            // println!("Within +/- 1 of y_loc_start/stop, {}, {}", (self.y_loc_start - other.y_loc_start).abs() < 2 ,( self.y_loc_stop - other.y_loc_start ).abs() < 2 );
            return true;
        } else if (self.y_loc_start <= other.y_loc_start) & ( other.y_loc_start <= self.y_loc_stop){
            // within y_start and stop
            // println!("Within y_loc_start/stop, {}, {}", self.y_loc_start >= other.y_loc_start , other.y_loc_start <= self.y_loc_stop);
            return true;
        }
        else{
            return false;
    
        }
    }
}

fn main() {

    let args  = env::args().collect::<Vec<String>>();

    // Read in a text file
    let filename = &args[1];

    // Parse lines to a vec of strings
    let my_vec = read_to_string(filename).unwrap().lines().map(|x| x.to_string()).collect::<Vec<String>>();

    let total_coords : Vec<Vec<Coord>> = Vec::new();

    let mut numeric_coords : Vec<Coord> = Vec::new();
    let mut non_numeric_coords : Vec<Coord> = Vec::new();
    

    for (i, line) in my_vec.iter().enumerate(){

        // let split_line = line.split(".").collect::<Vec<_>>();
        let char_list = line.chars();

        let mut coord_vec : Vec<Coord> = Vec::new();

        // println!("{}, {:?}", i, char_list);
        for (j, ch) in char_list.enumerate(){
            // if ch == '.' {
            //     continue;
            // }
            let coord = Coord{
                x_loc_start : i as i32,
                x_loc_stop : i as i32,
                y_loc_start : j as i32,
                y_loc_stop : j as i32,
                value : String::from(ch),
                numeric : ch.is_ascii_digit(),
                len : 1,
            };


            // println!("{:?}", coord);
            coord_vec.push(coord);
        }

        // Loop over and group adjacent numbers
        let mut temp_coord  = Coord::new();
        for coord in coord_vec.iter(){
            // println!("{:?}", coord);
            if !coord.numeric || coord.value == ".".to_string() {
                // println!("Not Numeric");

                if temp_coord.value == "".to_string() {
                    // println!("Empty");
                    non_numeric_coords.push(coord.clone());
                } else {
                    // println!("Finishing");
                    numeric_coords.push(temp_coord.clone());
                    non_numeric_coords.push(coord.clone());
                    temp_coord = Coord::new();
                }
            } else {
                if temp_coord.value == "".to_string() {
                    temp_coord = coord.clone();
                } else{
                    temp_coord.value += &coord.value;
                    temp_coord.len += 1;
                    temp_coord.y_loc_stop = coord.y_loc_stop;
                    
                }
            }
        }
        if temp_coord.numeric{
            numeric_coords.push(temp_coord.clone());
        } else if !temp_coord.value.is_empty(){
            non_numeric_coords.push(temp_coord.clone());

        }
    }


    non_numeric_coords = non_numeric_coords.iter().filter( |x| x.value != ".".to_string() ).map(|x| x.clone()).collect();
    let mut total = 0;

    for coord in numeric_coords.iter(){
        for ncoord in non_numeric_coords.iter(){
            if coord.is_adjacent(ncoord) {
                // println!("{:?}", coord);
                // // println!("{:?}", ncoord);
                // println!("[{}, {}, {}, '{}'] [{}, {}, {}, '{}']",
                // coord.x_loc_start, coord.y_loc_start, coord.y_loc_stop, coord.value,
                // ncoord.x_loc_start, ncoord.y_loc_start, ncoord.y_loc_stop, ncoord.value,
                    
                // );
                total += coord.value.parse::<i32>().unwrap();
                break;
            }
        }
    }

    // println!("{:?}", numeric_coords);
    println!("Total: {}", total);
    // Total: 542793
    // 542793
    // 544664


    // Part 2
    // Find numbers adjacent to stars
    let mut total_part2:i32 = 0;
    for ncoord in non_numeric_coords.iter(){
        if ncoord.value != "*".to_string(){
            continue;
        }
        let mut adj_nums: Vec<i32> = Vec::new();
        for coord in numeric_coords.iter(){
            if ncoord.is_adjacent(coord) {
                // println!("{:?}", coord);
                // println!("{:?}", ncoord);
                // println!("[{}, {}, {}, '{}'] [{}, {}, {}, '{}']",
                // coord.x_loc_start, coord.y_loc_start, coord.y_loc_stop, coord.value,
                // ncoord.x_loc_start, ncoord.y_loc_start, ncoord.y_loc_stop, ncoord.value,
                    
                // );
                adj_nums.push(coord.value.parse::<i32>().unwrap());
            }
        }
        if adj_nums.len() == 2{
            total_part2 += adj_nums[0] * adj_nums[1];
        }
    }

    println!("Total 2: {}", total_part2);
    // // println!("{:?}", non_numeric_coords );
    // for coord in numeric_coords.iter(){
    //     if coord.value == ".".to_string(){
    //         continue;
    //     }
    //     println!("{:?}", coord );
    // }

}
