use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::process;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Record {
    mass: u32,
}

fn process_input(file_path: String) -> Result<u32, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut rdr = csv::ReaderBuilder::new().has_headers(false).from_reader(reader);

    println!("Deserializing");

    let mut vec: Vec<u32> = vec![];

    for result in rdr.deserialize() {
        let record: Record = result?;

        let result: u32 = record.mass / 3 - 2;

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
