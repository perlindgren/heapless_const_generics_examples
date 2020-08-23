// Proof of concept

use heapless::Vec;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let point = Point { x: 1, y: 2 };

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&point).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a Point.
    let deserialized: Point = serde_json::from_str(&serialized).unwrap();

    // Prints deserialized = Point { x: 1, y: 2 }
    println!("deserialized = {:?}", deserialized);

    let v: Vec<u8, 8> = Vec::from_slice(&[1, 2, 3, 5, 8]).unwrap();
    println!("v {:?}", &v);

    let serialized = serde_json::to_string(&v).unwrap();
    println!("serialized = {}", serialized);

    let v: Vec<Point, 8> = Vec::from_slice(&[Point { x: 1, y: 2 }, Point { x: 1, y: 2 }]).unwrap();
    let serialized = serde_json::to_string(&v).unwrap();
    println!("serialized = {:?}", v);

    // Convert the JSON string back to a Vec<Point, 8>.
    let deserialized: Vec<Point, 8> = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);

    let write_file = File::create("examples/vec.cbor").unwrap();
    serde_cbor::to_writer(write_file, &v).unwrap();

    let read_file = File::open("examples/vec.cbor").unwrap();
    let v: Vec<Point, 8> = serde_cbor::from_reader(read_file).unwrap();
    println!("v = {:?}", v);
}

// fn main() -> Result<(), Box<dyn Error>> {
//     let ferris = Mascot {
//         name: "Ferris".to_owned(),
//         species: "crab".to_owned(),
//         year_of_birth: 2015,
//     };

//     let ferris_file = File::create("examples/ferris.cbor")?;
//     // Write Ferris to the given file.
//     // Instead of a file you can use any type that implements `io::Write`
//     // like a HTTP body, database connection etc.
//     serde_cbor::to_writer(ferris_file, &ferris)?;

//     let tux_file = File::open("examples/ferris.cbor")?;
//     // Load Tux from a file.
//     // Serde CBOR performs roundtrip serialization meaning that
//     // the data will not change in any way.
//     let tux: Mascot = serde_cbor::from_reader(tux_file)?;

//     println!("{:?}", tux);
//     // prints: Mascot { name: "Tux", species: "penguin", year_of_birth: 1996 }

//     Ok(())
// }
