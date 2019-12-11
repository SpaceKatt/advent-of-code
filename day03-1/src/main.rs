use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;
use std::str::FromStr;
use std::i32;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct Record {
    directions: Vec<String>,
}

struct Wire {
    directions: Vec<String>,
}

type WireBox = Vec<Wire>;

struct WireMemo {
    memo: Vec<bool>,
}

struct WireMomento {
    box_size: usize,
    momento: HashMap<String, WireMemo>,
}

struct PortCoordinate {
    x: i32,
    y: i32,
}

fn get_coordinate_hash(coord: &PortCoordinate) -> Result<String, Box<dyn Error>> {
    let x_hash = if coord.x < 0 {
        format!("-{:0>10}", i32::abs(coord.x).to_string())
    } else {
        format!("{:0>11}", coord.x.to_string())
    };

    let y_hash = if coord.x < 0 {
        format!("-{:0>10}", i32::abs(coord.y).to_string())
    } else {
        format!("{:0>11}", coord.y.to_string())
    };

    let hash = format!("{}{}", x_hash, y_hash);

    Ok(hash)
}

fn get_coordinate_from_hash(hash: String) -> Result<PortCoordinate, Box<dyn Error>> {
    let x = i32::from_str(&hash[..11]).unwrap();
    let y = i32::from_str(&hash[11..]).unwrap();
    let coord = PortCoordinate { x, y };

    Ok(coord)
}

fn parse_input(file_path: &str) -> Result<WireBox, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut rdr = csv::ReaderBuilder::new().has_headers(false).delimiter(b'\n').from_reader(reader);

    let mut wire_box: Vec<Wire> = vec![];

    for result in rdr.deserialize() {
        let record: Record = result?;

        for wire_directions in record.directions {
            let directions: Vec<String> = wire_directions.split(',').map(|x| String::from(x)).collect();

            if directions.len() < 1 || directions[0] == "" {
                continue;
            } 

            let wire = Wire {
                directions,
            };

            wire_box.push(wire);
        }
    }

    Ok(wire_box)
}

fn get_target(direction: String) -> Result<PortCoordinate, Box<dyn Error>> {
    let heading = &direction[0..1];
    let magnitude = i32::from_str(&direction[1..]).unwrap();

    Ok(match heading {
        "D" => PortCoordinate { x: 0, y: -magnitude},
        "U" => PortCoordinate { x: 0, y: magnitude},
        "L" => PortCoordinate { x: -magnitude, y: 0},
        "R" => PortCoordinate { x: magnitude, y: 0},
        _ => panic!("At the disco!"),
    })
}

fn traverse_wire(wire: &Wire, wire_id: u32, momento: &mut WireMomento) {
    let mut wire_pointer = PortCoordinate { x: 0, y: 0 };

    for path in wire.directions.iter() {
        let target = get_target(path.to_string()).unwrap();
        let is_horizontal = target.x != 0;

        let mut wire_ptr: i32 = if is_horizontal { wire_pointer.x } else { wire_pointer.y };
        let ptr: i32 = if is_horizontal { target.x } else { target.y };

        let target_num = wire_ptr + ptr;

        let iter = if ptr > 0 { wire_ptr..(target_num + 1) } else { target_num..(wire_ptr + 1) };

        for step in iter {
            let current_step =  PortCoordinate {
                x: if is_horizontal { wire_ptr } else { wire_pointer.x},
                y: if !is_horizontal { wire_ptr } else { wire_pointer.y},
            };

            let key = get_coordinate_hash(&current_step).unwrap();


            if !momento.momento.contains_key(&key) {
                let fresh_memo = WireMemo { memo: vec![false; momento.box_size] };

                momento.momento.insert(key.to_string(), fresh_memo);
            }

            let mut wire_memo = momento.momento.get(&key).unwrap().memo.to_vec();

            wire_memo[wire_id as usize] = true;


            momento.momento.insert(key.to_string(),  WireMemo { memo: wire_memo });

            wire_ptr = step;
        }

        wire_pointer = PortCoordinate {
            x: if is_horizontal { target_num } else { wire_pointer.x},
            y: if !is_horizontal { target_num } else { wire_pointer.y},
        };
    }
}

fn find_shortest_intersection_distance(intersections: Vec<String>, cost_fn: &dyn Fn(&PortCoordinate, &PortCoordinate) -> i32) -> i32 {
    let origin = PortCoordinate {
        x: 0,
        y: 0,
    };

    let mut min_cost: i32 = i32::MAX;

    for intersection in intersections {
        let coord = get_coordinate_from_hash(intersection).unwrap();
        let cost = cost_fn(&coord, &origin);

        if cost <= min_cost && cost > 0 {
            min_cost = cost;
        }
    }

    min_cost
}

fn main() {
    let wire_box: WireBox = parse_input("./src/directions.csv").unwrap();
    let momento_map: HashMap<String, WireMemo> = HashMap::new();

    let mut momento = WireMomento {
        box_size: wire_box.len(),
        momento: momento_map,
    };

    for (wire_id, wire) in wire_box.iter().enumerate() {
        traverse_wire(wire, wire_id as u32, &mut momento);
    }

    let mut intersections: Vec<String> = vec![];

    for key in momento.momento.keys() {
        let memo = momento.momento.get(key).unwrap();

        let intersection = memo.memo.iter().fold(true, |a, b| a & b);

        if intersection {
            intersections.push(String::from(key));
        }
    }

    let manhattan_dist = |x: &PortCoordinate, y: &PortCoordinate| {
        let result: i32 = i32::abs(x.x - y.x) + i32::abs(y.y - x.y);

        result
    };

    let result = find_shortest_intersection_distance(intersections, &manhattan_dist);

    println!("Final :: {}", result);
}
