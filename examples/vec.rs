use heapless::Vec;
use serde::{Deserialize, Serialize};

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
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a Vec<Point, 8>.
    let deserialized: Vec<Point, 8> = serde_json::from_str(&serialized).unwrap();
    println!("deserialized = {:?}", deserialized);
}
