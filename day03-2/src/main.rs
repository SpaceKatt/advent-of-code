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

type Wire = Vec<String>;

type WireBox = Vec<Wire>;

type MemoTuple = (bool, i32);
type WireMemo = Vec<MemoTuple>;

struct WireMomento {
    box_size: usize,
    momento: HashMap<String, WireMemo>,
}

struct PortCoordinate {
    x: i32,
    y: i32,
}

fn get_coordinate_hash(coord: &PortCoordinate) -> Result<String, Box<dyn Error>> {
    let format_int = |x: i32| {
        if x < 0 {
            format!("-{:0>10}", i32::abs(x).to_string())
        } else {
            format!("{:0>11}", x.to_string())
        }
    };

    let hash = format!("{}{}", format_int(coord.x), format_int(coord.y));

    Ok(hash)
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

            wire_box.push(directions);
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
    let mut total_dist = 0;

    for path in wire.iter() {
        let target = get_target(path.to_string()).unwrap();
        let is_horizontal = target.x != 0;

        let mut wire_ptr: i32 = if is_horizontal { wire_pointer.x } else { wire_pointer.y };
        let ptr: i32 = if is_horizontal { target.x } else { target.y };

        let target_num = wire_ptr + ptr;

        let gen_seq = |start: i32, end: i32| {
            let mut range: Vec<i32> = Vec::with_capacity(i32::abs(start - end) as usize);
            let increment = if start < end { 1 } else { -1 };
            let mut current = start;

            while current != end {
                range.push(current);
                current += increment;
            }

            range
        };

        let actual_end = if target_num < wire_ptr { target_num - 1 } else { target_num + 1};

        for step in gen_seq(wire_ptr, actual_end).iter() {
            wire_ptr = *step;

            let current_step =  PortCoordinate {
                x: if is_horizontal { wire_ptr } else { wire_pointer.x},
                y: if !is_horizontal { wire_ptr } else { wire_pointer.y},
            };

            let key = get_coordinate_hash(&current_step).unwrap();


            if !momento.momento.contains_key(&key) {
                let fresh_memo: WireMemo = vec![(false, 0); momento.box_size];

                momento.momento.insert(key.to_string(), fresh_memo);
            }

            let mut wire_memo = momento.momento.get(&key).unwrap().to_vec();

            let previos_dist = wire_memo[wire_id as usize].1;

            if previos_dist == 0 {
                wire_memo[wire_id as usize] = (true, total_dist);
            } else {
                wire_memo[wire_id as usize] = (true, previos_dist);
            }

            momento.momento.insert(key.to_string(), wire_memo);

            total_dist += 1;
        }

        total_dist -= 1;

        wire_pointer = PortCoordinate {
            x: if is_horizontal { target_num } else { wire_pointer.x},
            y: if !is_horizontal { target_num } else { wire_pointer.y},
        };
    }
}

fn find_shortest_intersection_distance(intersections: Vec<(String, i32)>) -> i32 {
    let mut min_cost: i32 = i32::MAX;

    for intersection in intersections {
        let cost: i32 = intersection.1;

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

    let mut intersections: Vec<(String, i32)> = vec![];

    for key in momento.momento.keys() {
        let memo = momento.momento.get(key).unwrap();

        let intersection = memo.iter().fold((true, 0), |(a, x), (b, y)| (a & b, x + y));

        if intersection.0 {
            intersections.push((String::from(key), intersection.1));
        }
    }

    let result = find_shortest_intersection_distance(intersections);

    println!("Final :: {}", result);
}
