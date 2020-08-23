// Proof of concept

use heapless::Vec;
use memory_lane::common::*;
use std::fs::File;

fn main() {
    // let read_file = File::open("examples/mem.cbor").unwrap();
    // let v: Vec<Point, 8> = serde_cbor::from_reader(&read_file).unwrap();
    // println!("v = {:?}", v);
    // let v: Vec<Point, 8> = serde_cbor::from_reader(&read_file).unwrap();
    // println!("v = {:?}", v);
    // // let v: Vec<(u8, u8), 8> = serde_cbor::from_reader(&read_file).unwrap();
    // // println!("v = {:?}", v);
    let read_file = File::open("examples/mem.cbor").unwrap();
    let v: Point = serde_cbor::from_reader(&read_file).unwrap();
    println!("v = {:?}", v);
    let v: Point = serde_cbor::from_reader(&read_file).unwrap();
    println!("v = {:?}", v);
}
