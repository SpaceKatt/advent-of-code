use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::process;
use std::str::FromStr;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Record {
    opcodes: String,
}

fn parse_input(file_path: String) -> Result<Vec<u32>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut rdr = csv::ReaderBuilder::new().delimiter(b'\'').has_headers(false).from_reader(reader);

    let mut vec: Vec<u32> = vec![];

    for result in rdr.deserialize() {
        let record: Record = result?;

        let stuff: Vec<&str> = record.opcodes.split(',').collect();

        for thing in stuff {
            vec.push(u32::from_str(thing).unwrap());
        }
    }

    Ok(vec)
}

fn process_program(opcodes: &mut Vec<u32>) -> Result<u32, Box<dyn Error>> {
    let mut next_position: usize = 0;

    opcodes[1] = 12;
    opcodes[2] = 2;

    loop {
        let opcode = opcodes[next_position];
        let first = opcodes[next_position + 1];
        let second = opcodes[next_position + 2];
        let dest = opcodes[next_position + 3];

        let operand_left = opcodes[first as usize];
        let operand_right = opcodes[second as usize];

        let result = match opcode {
            1 => operand_left + operand_right,
            2 => operand_left * operand_right,
            99 => break,
            _ => panic!("Unknown opcode: {}", opcode),
        };

        opcodes[dest as usize] = result;

        next_position += 4;
    }

    Ok(opcodes[0])
}

fn main() {
    let mut input = match parse_input(String::from("./src/opcodes.csv")) {
        Ok(vector) => vector,
        Err(e) => {
            println!("Error: {}", e);
            process::exit(1);
        }
    };

    match process_program(&mut input) {
        Ok(number) => println!("Final result :: {}", number),
        Err(e) => {
            println!("Error: {}", e);
            process::exit(1);
        }
    }
}
