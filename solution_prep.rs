use std::io;

fn main() {
    println!("Enter the desired concentration of the solution (in g/L):");

    let mut concentration = String::new();
    io::stdin().read_line(&mut concentration)
        .expect("Failed to read line");

    let concentration: f64 = match concentration.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Invalid input"),
    };

    println!("Enter the volume of the solution (in liters):");

    let mut volume = String::new();
    io::stdin().read_line(&mut volume)
        .expect("Failed to read line");

    let volume: f64 = match volume.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Invalid input"),
    };

    let mass = concentration * volume;

    println!("To prepare a {} L solution with a concentration of {} g/L, you will need {} g of solute",
             volume, concentration, mass);
}
