use std::io;

fn main() {
    println!("Enter the volume of the solution (in liters):");

    let mut volume = String::new();
    io::stdin().read_line(&mut volume)
        .expect("Failed to read line");

    let volume: f64 = match volume.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Invalid input"),
    };

    println!("Enter the mass of the solute (in grams):");

    let mut mass = String::new();
    io::stdin().read_line(&mut mass)
        .expect("Failed to read line");

    let mass: f64 = match mass.trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Invalid input"),
    };

    let concentration = mass / volume;

    println!("The concentration of the solution is {} g/L", concentration);
}
