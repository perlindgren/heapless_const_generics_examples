// Common

use serde::{Deserialize, Serialize};
use serde_cbor::Value;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct NoPoint {
    pub x: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Cbor {
    Native(Value),
    Point(Point),
    NoPoint(NoPoint),
}

// use memory_lane::Lane;
impl crate::LaneId for Point {
    fn id(&self) -> u32 {
        0;
    }
}
