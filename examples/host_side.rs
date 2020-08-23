// Proof of concept

use heapless::Vec;
use memory_lane::common::*;
use serde_cbor::{de::SliceRead, Deserializer};
use std::fs::File;
use std::io::prelude::*;

fn main() {
    // emulate host reading from memory
    let mut file = File::open("examples/mem.raw").unwrap();
    let mut buf = [0u8; 100];
    file.read(&mut buf).unwrap();

    let mut deserializer = Deserializer::from_slice(&buf);
    let v1: Vec<Point, 8> = serde::de::Deserialize::deserialize(&mut deserializer).unwrap();
    println!("v1 = {:?}", v1);
    let v2: Vec<Point, 8> = serde::de::Deserialize::deserialize(&mut deserializer).unwrap();
    println!("v2 = {:?}", v2);
}
