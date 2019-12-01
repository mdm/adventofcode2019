use std::io::BufRead;

fn main() {
    let mut total_fuel = 0;
    let mut total_additional_fuel = 0;
    for line in std::io::stdin().lock().lines() {
        let mass: i32 = line.unwrap().parse().unwrap();

        let fuel = mass / 3 - 2;
        total_fuel += fuel;

        let mut additional_fuel = fuel / 3 - 2;
        while additional_fuel > 0 {
            total_additional_fuel += additional_fuel;
            additional_fuel = additional_fuel / 3 - 2;
        }
    }

    println!("{}", total_fuel);
    println!("{}", total_fuel + total_additional_fuel);
}
