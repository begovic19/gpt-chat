use std::io;

fn main() {
    println!("Enter the concentration of reactant A:");

    let mut reactant_a = String::new();
    io::stdin().read_line(&mut reactant_a)
        .expect("Failed to read line");

    let reactant_a: f64 = match reactant_a.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Invalid input"),
    };

    println!("Enter the concentration of reactant B:");

    let mut reactant_b = String::new();
    io::stdin().read_line(&mut reactant_b)
        .expect("Failed to read line");

    let reactant_b: f64 = match reactant_b.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Invalid input"),
    };

    println!("Enter the rate constant for the reaction:");

    let mut rate_constant = String::new();
    io::stdin().read_line(&mut rate_constant)
        .expect("Failed to read line");

    let rate_constant: f64 = match rate_constant.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Invalid input"),
    };

    let reaction_rate = rate_constant * reactant_a * reactant_b;

    println!("The reaction rate is {}", reaction_rate);
}
