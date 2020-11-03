use std::io;

fn main() {
    let mut input = String::new();
    println!("Please enter your weight (kg): ");
    io::stdin().read_line(&mut input).unwrap();
    let weight: f32 = input.trim().parse().unwrap();

    let mut mars_weight = calculate_weight(weight); 
    mars_weight = mars_weight * 1000.0;
    println!("Weight on Mars {}", mars_weight);
}//main

fn calculate_weight(weight: f32) -> f32 {
    (weight / 9.81) * 3.711
}

