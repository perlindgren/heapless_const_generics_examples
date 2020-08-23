// Common

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}
