// Use fs to read in file
use std::fs::read_to_string;
// Use env to get the command line arguments
use std::env;

// Recursive find string
fn find_string(s: String, sub_str: &str) -> (Vec<String>, Vec<i32>){

    // store values and the location within vectors
    let mut vals: Vec<String>  = Vec::new();
    let mut locations : Vec<i32>  = Vec::new();

    // Check if the string contains the sub string 
    // Returns an option
    let res = s.find(sub_str);
    match res {

        // Match on if found
        Some(x) => {
            // Add the value to the vectors
            vals.push(sub_str.to_string());
            locations.push(x as i32 +1);

            // Recursive search for this substring
            // Start from the next character
            // i = oneeight -> One, i+1 = neeight -> eight
            let (tmp_vals, tmp_locations) = find_string(s[(x+1)..].to_string(), &sub_str);
            
            for i in 0..tmp_vals.len(){
                // Add values to the vectors
                vals.push(tmp_vals[i].clone());
                // Adjust the location
                locations.push(x as i32 + tmp_locations[i] + 1);
            }
        }
        // If not found return vectors
        None => {
            return (vals, locations)
        }
    }
    (vals, locations)
    
}


// Function to loop over digits
fn find_string_numbers(s: String) -> (Vec<String>, Vec<i32>){
    let digits = [
        "one",
        "two",
        "three",
        "four",
        "five",
        "six",
        "seven",
        "eight",
        "nine",
    ];

    let mut vals: Vec<String>  = Vec::new();
    let mut locations : Vec<i32>  = Vec::new();

    // Loop over digits
    for (i, dig) in digits.iter().enumerate(){
        // Get vectors of the values and the locations
        let (tmp_vals, tmp_locations) = find_string(s.clone(), dig);
        for j in 0..tmp_vals.len() {
            vals.push((i + 1).to_string());
            locations.push(tmp_locations[j]-1);
        }
    }
    return (vals, locations);
}


// Function to search for a numberic character
fn get_number(s : String) -> (Vec<String>, Vec<i32>){
    let mut nums : Vec<String> = Vec::new();
    let mut locations : Vec<i32> = Vec::new();

    // Enumerate to get the index
    for (i, c) in s.chars().enumerate(){
        // println!("{}, {}", c, c.is_ascii_digit());
        // Check if it is a digit
        if c.is_ascii_digit(){
            // println!("{}", c);
            nums.push(c.to_string());
            // Cast to i32
            locations.push(i as i32);
        }
    }
    
    return (nums, locations);

}

fn main() {
    let args  = env::args().collect::<Vec<String>>();

    // Read in a text file
    let filename = &args[1];

    // Parse lines to a vec of strings
    let my_vec = read_to_string(filename).unwrap().lines().map(|x| x.to_string()).collect::<Vec<String>>();

    // Sum
    let mut sum = 0;

    // loop over each line 
    for i in 0..my_vec.len(){

        // Part 1 find the number in chars 
        let n: (Vec<String>, Vec<i32>) = get_number(my_vec[i].clone());
        // println!("{:?},{:?}", n.0, n.1);
        let mut combined_str : Vec<String> = Vec::new();
        let mut combined_i32 : Vec<i32> = Vec::new();

        // Add to final combined vecs
        for j in 0..n.0.len(){
            combined_str.push(n.0[j].clone());
            combined_i32.push(n.1[j]);
        }

        // Gind the written numbers
        let x: (Vec<String>, Vec<i32>) = find_string_numbers(my_vec[i].clone());
        // Add to combined vecs
        for j in 0..x.0.len(){
            combined_str.push(x.0[j].clone());
            combined_i32.push(x.1[j]);
        }

        // Find the first and the last
        let mut first = "".to_string();
        let mut last = "".to_string();
        let mut i_first = 999;
        let mut i_last = -999;
        // Could of been in the first loop!
        for k in 0..combined_i32.len(){
            if combined_i32[k] < i_first{
                i_first = combined_i32[k];
                first = combined_str[k].clone();
            }
            
            if combined_i32[k] > i_last{
                i_last = combined_i32[k];
                last = combined_str[k].clone();
            }
        }

        // Combine the strings
        // Could be first * 10 + last
        let ret_string: String = first.clone() + &last;
        // println!("{}, {}", my_vec[i], my_vec[i].len());
        // println!("{:?}, {:?}", combined_str, combined_i32);
        // println!("{}", ret_string);
        // println!("{}, {}", first, last);

        sum += ret_string.parse::<i32>().unwrap();

    }

    println!("Sum: {}", sum);
}
