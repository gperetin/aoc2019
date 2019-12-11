use std::fs;

fn compute_fuel(mass: i32) -> i32 {
    let current_fuel = (mass / 3) - 2;
    if current_fuel <= 0 {
        return 0
    } else {
        return current_fuel + compute_fuel(current_fuel)
    }
}

fn main() {
    let input = fs::read_to_string("input.txt").expect("Unable to read inputs");
    let total_fuel: i32 = input
        .split_ascii_whitespace()
        .map(|mass| {
            let mass_int = mass.parse::<i32>().unwrap();
            compute_fuel(mass_int)
        }).sum();
    println!("{}", total_fuel);
}
