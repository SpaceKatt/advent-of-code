use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::process;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Record {
    mass: i32,
}

fn calculate_fuel_needed(mass: i32) -> i32 {
    let result = mass / 3 - 2;

    if result <= 0 {
        return 0;
    }

    return result + calculate_fuel_needed(result);
}

fn process_input(file_path: String) -> Result<i32, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut rdr = csv::ReaderBuilder::new().has_headers(false).from_reader(reader);

    println!("Deserializing");

    let mut vec: Vec<i32> = vec![];

    for result in rdr.deserialize() {
        let record: Record = result?;

        let result = calculate_fuel_needed(record.mass);

        vec.push(result);
    }

    let sum = vec.iter().fold(0, |x, y| x + y);

    println!("Finished");

    Ok(sum)
}

fn main() {
    let result = process_input(String::from("./src/masses.csv"));

    match result {
        Ok(v) => println!("{}", v),
        Err(e) => {
            println!("Error processing input: {}", e);
            process::exit(1);
        },
    }
}
