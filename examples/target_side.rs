// Proof of concept

use heapless::Vec;
use memory_lane::common::*;
use serde::Serialize;
use serde_cbor::{ser::SliceWrite, Serializer};
use std::fs::File;
use std::io::prelude::*;

fn main() {
    let v: Vec<_, 8> = Vec::from_slice(&[Point { x: 1, y: 2 }, Point { x: 1, y: 2 }]).unwrap();
    let mut buf = [0u8; 100];
    let writer = SliceWrite::new(&mut buf[..]);
    let mut ser = Serializer::new(writer).packed_format();
    v.serialize(&mut ser).unwrap();
    println!("ser {:?}", &ser);
    v.serialize(&mut ser).unwrap();
    println!("ser {:?}", &ser);

    let buf = ser.into_inner().into_inner();

    // emulate write to memory
    let mut file = File::create("examples/mem.raw").unwrap();
    file.write_all(buf).unwrap();
}
