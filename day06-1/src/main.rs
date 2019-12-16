use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::collections::HashMap;

use serde::Deserialize;

type Orbits = Vec<String>;

struct Planet {
    satellites: Vec<String>,
}

#[derive(Deserialize, Debug)]
struct Record {
    orbital: String,
}

fn parse_input(file_path: &str) -> Result<Orbits, Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);
    let mut rdr = csv::ReaderBuilder::new().has_headers(false).from_reader(reader);

    let mut orbits: Orbits = vec![];

    for result in rdr.deserialize() {
        let record: Record = result?;

        orbits.push(record.orbital);
    }

    Ok(orbits)
}

fn build_planet_map(orbits: Orbits) -> HashMap<String, Planet> {
    let mut map: HashMap<String, Planet> = HashMap::new();

    for orbit in orbits {
        let planets: Vec<&str> = orbit.split(')').collect();

        let center = String::from(planets[0]);
        let satellite = String::from(planets[1]);

        if map.contains_key(&center) {
            let satellite_vec = map.get_mut(&center).unwrap();
            satellite_vec.satellites.push(satellite);
        } else {
            map.insert(center.clone(), Planet {
                satellites: vec![satellite.clone()],
            });
        }
    }

    map
}

fn traverse_orbital_tree(planet_map: HashMap<String, Planet>) -> i32 {
    fn traverse_from_node(planet_name: String, depth: i32, map: &HashMap<String, Planet>) -> i32 {
        let planet = map.get(&planet_name);

        match planet {
            Some(planet) => {
                let mut child_orbit_count = 0;

                for satellite in planet.satellites.iter() {
                    child_orbit_count += traverse_from_node(satellite.clone(), depth + 1, &map);
                }
        
                child_orbit_count + depth
            },
            _ => depth,
        }
    };

    traverse_from_node(String::from("COM"), 0, &planet_map)
}

fn main() {
    let orbits = parse_input("./src/orbits.csv").unwrap();
    let planet_map = build_planet_map(orbits);

    let result = traverse_orbital_tree(planet_map);

    println!("{}", result);
}
