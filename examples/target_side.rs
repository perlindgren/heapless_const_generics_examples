// Proof of concept

use heapless::Vec;
use memory_lane::common::*;
use std::fs::File;

fn main() {
    // let v: Vec<Point, 8> = Vec::from_slice(&[Point { x: 1, y: 2 }, Point { x: 1, y: 2 }]).unwrap();
    // let write_file = File::create("examples/mem.cbor").unwrap();
    // serde_cbor::to_writer(&write_file, &v).unwrap();
    // serde_cbor::to_writer(&write_file, &v).unwrap();
    // // let v: Vec<(u8, u8), 8> = Vec::from_slice(&[(1, 2), (3, 4)]).unwrap();
    // // serde_cbor::to_writer(&write_file, &v).unwrap();

    let v = Point { x: 1, y: 1 };
    let write_file = File::create("examples/mem.cbor").unwrap();
    serde_cbor::to_writer(&write_file, &v).unwrap();
    serde_cbor::to_writer(&write_file, &v).unwrap();
}
