// Proof of concept
#![feature(core_intrinsics)]

use core::intrinsics::type_id;

use memory_lane::common::*;
use serde::Serialize;
use serde_cbor::{ser::SliceWrite, value::to_value, Serializer, Value};
use std::any::TypeId;
use std::convert::Into;
use std::fs::File;
use std::io::prelude::*;

fn trace<S>(ser: &mut Serializer<SliceWrite>, data: &S)
where
    S: Serialize,
{
    let v = serde_cbor::value::to_value(data);
    println!("v: {:?}", v);
    data.serialize(&mut *ser).unwrap();
}

fn main() {
    let mut buf = [0u8; 100];
    let writer = SliceWrite::new(&mut buf[..]);
    let mut ser = Serializer::new(writer).packed_format();

    let v = Point { x: 1, y: 2 };
    trace(&mut ser, &v);
    println!("ser {:?}", &ser);
    trace(&mut ser, &v);
    println!("ser {:?}", &ser);

    let some_u32 = 7;
    let v = serde_cbor::value::to_value(some_u32);
    println!("value {:?}", v);
    some_u32.serialize(&mut ser).unwrap();
    println!("ser {:?}", &ser);

    let buf = ser.into_inner().into_inner();

    // emulate write to memory
    let mut file = File::create("examples/mem.raw").unwrap();
    file.write_all(buf).unwrap();
}
