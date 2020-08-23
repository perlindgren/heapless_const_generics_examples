// Proof of concept
#![feature(core_intrinsics)]

use heapless::Vec;
use memory_lane::common::*;

use core::intrinsics::type_id;
use std::any::TypeId;
use std::convert::{From, Into};

// Does not work as S needs to be 'static
// fn trace<S>(data: S) {
//     let id = TypeId::of::<S>();
//     println!("typeid {:?}", id);
// }

fn main() {
    let v: Vec<_, 8> = Vec::from_slice(&[Point { x: 1, y: 2 }, Point { x: 1, y: 2 }]).unwrap();
    println!("typeid {:?}", TypeId::of::<String>());
    println!("typeid {:?}", TypeId::of::<Vec<Point, 8>>());
    let tid = type_id::<String>();
    println!("id {}", tid);
    //   let tid: std::num::NonZeroU64 = TypeId::of::<String>().into();
}
