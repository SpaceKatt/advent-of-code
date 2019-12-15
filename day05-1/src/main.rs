use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::process;
use std::str::FromStr;
use std::io;
use std::i32;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Record {
    opcodes: String,
}

fn parse_input(file_path: String) -> Result<Vec<i32>, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut rdr = csv::ReaderBuilder::new().delimiter(b'\'').has_headers(false).from_reader(reader);

    let mut vec: Vec<i32> = vec![];

    for result in rdr.deserialize() {
        let record: Record = result?;

        let stuff: Vec<&str> = record.opcodes.split(',').collect();

        for thing in stuff {
            vec.push(i32::from_str(thing).unwrap());
        }
    }

    Ok(vec)
}

fn get_input() -> i32 {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().parse().unwrap()
}

fn process_program(opcodes: &mut Vec<i32>) {
    let mut next_position: usize = 0;

    let get_nth_parameter_mode = |parameter_modes: i32, n: i32| -> i32{
        let magnitude = i32::pow(10, (n - 1) as u32);
        let _mod = i32::pow(10, n as u32);

        (parameter_modes / magnitude) % _mod
    };

    let get_dest = |target: usize, parameter_mode: i32, ops: &Vec<i32>| {
        match parameter_mode {
            0 => ops[target],
            1 => target as i32,
            _ => panic!("Unknown parameter :: {}", parameter_mode),
        }
    };

    loop {
        let instruction = opcodes[next_position];
        let opcode = instruction % 100;
        let parameter_modes = instruction / 100;

        let first_parameter_mode = get_nth_parameter_mode(parameter_modes, 1);
        let first = get_dest(next_position + 1, first_parameter_mode, opcodes);

        let second_parameter_mode = get_nth_parameter_mode(parameter_modes, 2);
        let second = get_dest(next_position + 2, second_parameter_mode, opcodes);

        let third_parameter_mode = get_nth_parameter_mode(parameter_modes, 3);
        let dest = get_dest(next_position + 3, third_parameter_mode, opcodes);

        match opcode {
            1 => {
                let operand_left = opcodes[first as usize];
                let operand_right = opcodes[second as usize];

                next_position += 4;
                opcodes[dest as usize] = operand_left + operand_right;
            },
            2 => {
                let operand_left = opcodes[first as usize];
                let operand_right = opcodes[second as usize];

                next_position += 4;
                opcodes[dest as usize] = operand_left * operand_right;
            },
            3 => {
                next_position += 2;
                let input = get_input();

                opcodes[first as usize] = input;
            },
            4 => {
                next_position += 2;
                println!("{}", opcodes[first as usize]);
            },
            99 => break,
            _ => panic!("Unknown opcode: {}", opcode),
        };
    }
}

fn main() {
    let mut input = match parse_input(String::from("./src/opcodes.csv")) {
        Ok(vector) => vector,
        Err(e) => {
            println!("Error: {}", e);
            process::exit(1);
        }
    };

    process_program(&mut input);
}
