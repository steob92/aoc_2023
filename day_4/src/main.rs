// Use fs to read in file
use std::fs::read_to_string;
// Use env to get the command line arguments
use std::env;


fn copy_winners(indx: usize, max_indx: usize, input_points : &Vec<i32>) -> Vec<i32> {
    let mut return_vec : Vec<i32> = Vec::new();
    // println!("{:?}", &input_points[indx..]);
    if input_points[indx] > 0 {
        // println!("{}, {}->{}", input_points[indx], indx+1, indx + input_points[indx] as usize +1);
        for i in (indx+1)..(indx + input_points[indx] as usize +1 ){
            return_vec.push(i as i32)
        }
    }
    return_vec

}

fn main() {

    let args  = env::args().collect::<Vec<String>>();

    // Read in a text file
    let filename = &args[1];

    // Parse lines to a vec of strings
    let my_vec = read_to_string(filename).unwrap().lines().map(|x| x.to_string()).collect::<Vec<String>>();

    for p in my_vec.iter(){
        println!("{:?}", p);
    }

    let mut winning_numbers: Vec<Vec<i32>> = Vec::new();
    let mut card_numbers: Vec<Vec<i32>> = Vec::new();
    my_vec.iter().for_each(|x|{
        let y = x.split(":").collect::<Vec<_>>()[1];
        let z = y.split("|").collect::<Vec<_>>();
        winning_numbers.push(z[0].split_ascii_whitespace().map(|s| s.parse::<i32>().unwrap() ).collect::<Vec<i32>>());
        card_numbers.push(z[1].split_ascii_whitespace().map(|s| s.parse::<i32>().unwrap() ).collect::<Vec<i32>>());
    });

    // println!("{}, {:?}", winning_numbers.len(), winning_numbers);
    // println!("{}, {:?}", card_numbers.len(), card_numbers);

    let mut total = 0; 

    // for part 2
    let mut card_points: Vec<i32> = vec![0_i32;winning_numbers.len()];

    for (i, (win, card)) in winning_numbers.iter().zip(card_numbers.iter()).enumerate(){
        let mut inner_total = 0;
        let mut card_match = 0;
        for c in card.iter(){
            if win.contains(c){
                card_match += 1;
                if inner_total == 0{
                    inner_total += 1;
                } else{
                    inner_total *= 2;
                }
            }
        }
        card_points[i] = card_match;

        total += inner_total;
    }

    println!("Total = {}", total);

    let mut total_cards: Vec<i32> = Vec::new();


    let mut card_vec : Vec<i32> = card_points.iter().enumerate().filter(|(i, &x)| x > 0).map(|(i, &x)| i as i32).collect::<Vec<i32>>();

    println!("{:?}", card_vec);
    let mut jindx = 0;
    while jindx < card_vec.len(){
        let mut tmp_vec: Vec<i32> = copy_winners(card_vec[jindx] as usize, card_points.len(), &card_points);
        card_vec.append(&mut tmp_vec);
        jindx += 1;
    }
    // println!("{:?}", copy_winners(0, 5, &card_points));
    // println!("{:?}", copy_winners(1, 5, &card_points));
    // println!("{:?}", copy_winners(2, 5, &card_points));
    // println!("{:?}", copy_winners(3, 5, &card_points));
    // println!("{:?}", copy_winners(4, 5, &card_points));
    // println!("{:?}", copy_winners(5, 5, &card_points));

    // println!("{:?}", card_vec);
    println!("{:?}", card_vec.len() + card_points.iter().filter(|&&x| x == 0).map(|_| 1).sum::<usize>());
    
    
}


